use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::dtos::user::{UpdateCenterForm, UpdateEmailForm, UpdatePassForm, UserForm, UserQuery};
use crate::models::SysUser;

pub struct UserController;

impl UserController {
    pub async fn page(query: web::Query<UserQuery>) -> impl Responder {
        let data = SysUser::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn save(form: web::Json<UserForm>) -> impl Responder {
        SysUser::save(form.0).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update(form: web::Json<UserForm>) -> impl Responder {
        let form = form.0;
        if form.id.is_none() {
            return RespBuilder::ok().with_msg("ID为空").build();
        }
        SysUser::update(form).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn delete_batch(ids: web::Json<Vec<i64>>) -> impl Responder {
        let ids = ids.0;
        SysUser::delete_batch(ids).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn center(form: web::Json<UpdateCenterForm>) -> impl Responder {
        let form = form.0;
        // TODO 获取用户ID
        let user_id = 0;
        SysUser::center(user_id, form).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update_avatar(_payload: Multipart) -> impl Responder {
        // TODO 获取用户ID
        // TODO 修改头像
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update_email(
        _code: web::Path<String>,
        form: web::Json<UpdateEmailForm>,
    ) -> impl Responder {
        let form = form.0;
        // TODO 获取用户ID
        let user_id = 0;
        // TODO 验证密码
        // TODO 验证验证码有效
        SysUser::update_email(user_id, form.email.unwrap_or_default()).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update_pass(form: web::Json<UpdatePassForm>) -> impl Responder {
        let form = form.0;
        // TODO 获取用户ID
        let user_id = 0;
        // TODO 验证密码
        SysUser::update_pass(user_id, form.new_pass).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn export() -> impl Responder {
        let export_list = SysUser::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("菜单");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row![
                "用户名",
                "角色",
                "部门",
                "岗位",
                "邮箱",
                "状态",
                "手机号码",
                "修改密码的时间",
                "创建日期"
            ])?;

            export_list.into_iter().for_each(|el| {
                let username = el.username.unwrap_or_default();
                let role = el.role.unwrap_or_default();
                let dept = el.dept.unwrap_or_default();
                let job = el.job.unwrap_or_default();
                let email = el.email.unwrap_or_default();
                let enabled = el.enabled.unwrap_or_default();
                let phone = el.phone.unwrap_or_default();
                let pwd_reset_time = el.pwd_reset_time.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![
                    username,
                    role,
                    dept,
                    job,
                    email,
                    enabled,
                    phone,
                    pwd_reset_time,
                    create_time
                ]);
            });
            Ok(())
        })
            .expect("write excel error!");
        let data = wb.close().expect("close excel error!");

        if data.is_none() {
            return HttpResponse::InternalServerError().finish();
        }

        HttpResponse::Ok()
            .header("Content-Type", "application/octet-stream")
            .header(
                "Content-Disposition",
                format!(
                    "attachment;filename={}菜单数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
