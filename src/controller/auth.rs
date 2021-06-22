use std::collections::HashSet;

use actix_session::Session;
use actix_web::{HttpRequest, Responder, web};
use captcha::{Captcha, Geometry};
use captcha::filters::{Cow, Noise, Wave};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use redis::{Commands, RedisResult};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::commons::{CURRENT_USER_SESSION_KEY, ONLINE_KEY, RespBuilder};
use crate::dtos::user::{CurrentUser, LoginDto, LoginForm, UserDto};
use crate::models::SysUser;
use crate::utils::{captcha_util, redis_util};

pub struct AuthController;

impl AuthController {
    pub async fn code() -> impl Responder {
        let mut captcha = captcha_util::captcha();
        let code = captcha.chars_as_string();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let mut map = Map::new();
        map.insert("uuid".to_string(), Value::from(uuid.as_str()));
        map.insert(
            "img".to_string(),
            Value::String(format!(
                "{}{}",
                "data:image/png;base64,",
                base64::encode(captcha.as_png().unwrap())
            )),
        );

        let set_result = redis_util::set(uuid.as_str(), &code.to_lowercase(), Some(60));
        if let Err(e) = set_result {
            return RespBuilder::fail()
                .with_msg("获取验证码失败，请重试")
                .build();
        }

        RespBuilder::ok().with_data(map).build()
    }

    pub async fn login(
        req: HttpRequest,
        form: web::Json<LoginForm>,
        session: Session,
    ) -> impl Responder {
        if form.uuid.is_none() || form.code.is_none() {
            return RespBuilder::fail().with_msg("验证码为空").build();
        }
        let code_result = redis_util::get(form.uuid.as_ref().unwrap());

        if let Ok(c) = code_result {
            let form_code = form.code.as_ref().unwrap();
            if !c.to_lowercase().eq(form_code) {
                return RespBuilder::fail().with_msg("验证码不正确").build();
            }
        } else {
            return RespBuilder::fail()
                .with_msg("验证码已失效，请刷新验证码！")
                .build();
        }

        let sys_user = SysUser::get_by_username(form.username.as_ref().unwrap())
            .await
            .unwrap();
        if sys_user.is_none() {
            return RespBuilder::fail().with_msg("账号或密码错误").build();
        }
        let su = sys_user.unwrap();

        // TODO verify password
        if su.password != form.password {
            return RespBuilder::fail().with_msg("账号或密码错误").build();
        }

        // let private_key_base64_decode = base64::decode(RSA_PRIVATE_KEY).unwrap();
        // let private_key = RSAPrivateKey::from_pkcs8(&private_key_base64_decode).unwrap();
        // // let private_key =
        // //     RSAPrivateKey::new(&mut RSA_PRIVATE_KEY, 2048).expect("failed to generate a key");
        // let public_key = RSAPublicKey::from(&private_key);
        //
        // let padding = PaddingScheme::new_pkcs1v15_encrypt();
        // let dec_data_r = private_key.decrypt(padding, &su.password.as_ref().unwrap().as_bytes());
        // let dec_data = match dec_data_r {
        //     Ok(d) => d,
        //     Err(e) => vec![],
        // };
        //
        // if dec_data.is_empty() {
        //     return HttpResponse::Ok().json(Resp::<String>::error_with_msg(String::from(
        //         "账号或密码错误",
        //     )));
        // }

        let key = b"secret";

        let mut header = Header {
            alg: Algorithm::HS512,
            kid: Some("signing_key".to_owned()),
            ..Default::default()
        };

        let token = jsonwebtoken::encode(&header, &su, &EncodingKey::from_secret(key));
        if token.is_err() {
            return RespBuilder::fail().with_msg("登录失败").build();
        }
        let token = token.unwrap_or_default();
        let mut map = Map::new();

        let user_id = su.id.unwrap_or_default();

        map.insert(
            "user".to_string(),
            serde_json::to_value(Self::packaging_user_info(user_id, &su).await).unwrap(),
        );
        map.insert("token".to_string(), Value::String((&token).to_string()));

        // put the current user information into the session
        let current_user = CurrentUser {
            user_id: su.id.unwrap_or_default(),
            dept_id: su.dept_id,
            username: su.username.unwrap_or_default(),
            nick_name: su.nick_name,
            is_admin: su.is_admin,
            key: Option::from((&token).to_string()),
            ..Default::default()
        };
        let _ = session.set(CURRENT_USER_SESSION_KEY, &current_user);
        let _ = session.set("user_id", &current_user.user_id);

        // put the online user information into the redis
        let rs = redis_util::set(
            format!("{}{}", ONLINE_KEY, &token).as_str(),
            serde_json::to_string(&current_user).unwrap_or_default(),
            None,
        );

        RespBuilder::ok().with_data(map).build()
    }

    pub async fn user_info(session: Session) -> impl Responder {
        let current_user = session
            .get::<CurrentUser>(CURRENT_USER_SESSION_KEY)
            .unwrap();
        if let Some(cu) = current_user {
            let sys_user = SysUser::get_by_username(&cu.username).await.unwrap();
            if let Some(su) = sys_user {
                let data = Self::packaging_user_info(cu.user_id, &su).await;
                return RespBuilder::ok().with_data(data).build();
            }
        }
        RespBuilder::<LoginDto>::fail()
            .with_msg("用户不存在")
            .build()
    }

    async fn packaging_user_info(user_id: i64, su: &SysUser) -> LoginDto {
        // user's department
        let user_dept = SysUser::user_dept(user_id).await.unwrap_or_default();
        // user's job
        let user_job = SysUser::user_jobs(user_id).await.unwrap_or_default();
        // user's role
        let user_roles = SysUser::user_roles(user_id).await.unwrap_or_default();

        let user_dto = UserDto {
            id: su.id,
            username: su.username.clone(),
            nick_name: su.nick_name.clone(),
            gender: su.gender.clone(),
            phone: su.phone.clone(),
            email: su.email.clone(),
            avatar_name: su.avatar_name.clone(),
            avatar_path: su.avatar_path.clone(),
            enabled: su.enabled,
            create_by: su.create_by.clone(),
            update_by: su.update_by.clone(),
            create_time: su.create_time,
            update_time: su.update_time,
            dept: user_dept,
            jobs: user_job,
            roles: user_roles,
        };

        // user's data scopes
        let data_scopes = SysUser::user_data_scopes(user_id).await.unwrap_or_default();
        // user's menu permission
        let user_menu_permissions = SysUser::user_menu_permissions(user_id)
            .await
            .unwrap_or_default();

        LoginDto {
            data_scopes: Option::from(data_scopes),
            roles: {
                if let Some(is_admin) = su.is_admin {
                    if is_admin == 1 {
                        let mut hs = HashSet::with_capacity(1);
                        hs.insert("admin".to_string());
                        Some(hs)
                    } else {
                        Option::from(user_menu_permissions)
                    }
                } else {
                    Option::from(user_menu_permissions)
                }
            },
            user: Option::from(user_dto),
        }
    }

    pub async fn logout(req: HttpRequest, session: Session) -> impl Responder {
        let current_user = session
            .get::<CurrentUser>(CURRENT_USER_SESSION_KEY)
            .unwrap();
        if let Some(cu) = current_user {
            redis_util::del(&[format!("{}{:?}", ONLINE_KEY, cu.key).as_str()]);
        };
        RespBuilder::<&str>::ok().build()
    }
}
