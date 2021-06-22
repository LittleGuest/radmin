use std::collections::HashSet;

use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::dept::UserDeptDto;
use crate::dtos::job::UserJobDto;
use crate::dtos::role::UserRoleDto;
use crate::dtos::user::{UpdateCenterForm, UserExportDto, UserForm, UserQuery};
use crate::models::{SysDept, SysUsersJobs, SysUsersRoles};
use crate::models::RBatisModel;
use crate::RB;

/// 系统用户
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug, Eq, PartialEq)]
pub struct SysUser {
    /// ID
    pub id: Option<i64>,
    /// 部门名称
    pub dept_id: Option<i64>,
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nick_name: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 头像地址
    pub avatar_name: Option<String>,
    /// 头像真实路径
    pub avatar_path: Option<String>,
    /// 密码
    #[serde(skip_serializing)]
    pub password: Option<String>,
    /// 是否为admin账号
    pub is_admin: Option<u8>,
    /// 状态：1启用、0禁用
    pub enabled: Option<i64>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新着
    pub update_by: Option<String>,
    /// 修改密码的时间
    pub pwd_reset_time: Option<chrono::NaiveDateTime>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysUser {
    pub async fn get_by_username(username: &str) -> Result<Option<Self>> {
        let mut wr = RB.new_wrapper();
        wr.eq("username", username);
        wr.check()?;

        let user = RB.fetch_by_wrapper::<Option<Self>>("", &wr).await?;
        Ok(user)
    }

    pub async fn page(query: UserQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.blurry.is_some() {
            wr.and();
            wr.eq("nick_name", &query.blurry);
            wr.or();
            wr.eq("email", &query.blurry);
        }
        if query.dept_id.is_some() {
            wr.eq("dept_id", query.dept_id);
        }
        if query.enabled.is_some() {
            wr.eq("enabled", query.enabled);
        }
        wr.check()?;
        let pages: Page<Self> = RB
            .fetch_page_by_wrapper(
                "",
                &wr,
                &PageRequest::new(query.current.unwrap_or(1), query.size.unwrap_or(20)),
            )
            .await?;
        Ok(pages)
    }

    pub async fn save(form: UserForm) -> Result<Self> {
        let entity = Self {
            dept_id: form.dept_id,
            username: form.username,
            nick_name: form.nick_name,
            gender: form.gender,
            phone: form.phone,
            email: form.email,
            enabled: form.enabled,
            ..Default::default()
        };
        let dbr = RB.save("", &entity).await?;

        // 用户岗位
        if let Some(jobs) = form.jobs {
            let ujs = jobs.iter().fold(Vec::new(), |mut ujs, job_id| {
                let uj = SysUsersJobs {
                    user_id: dbr.last_insert_id,
                    job_id: Option::from(*job_id),
                };
                ujs.push(uj);
                ujs
            });

            SysUsersJobs::save_batch(&ujs).await;
        }
        // 用户角色
        if let Some(roles) = form.roles {
            let urs = roles.iter().fold(Vec::new(), |mut urs, role_id| {
                let ur = SysUsersRoles {
                    user_id: dbr.last_insert_id,
                    role_id: Option::from(*role_id),
                };
                urs.push(ur);
                urs
            });
            SysUsersRoles::save_batch(&urs).await;
        }

        Ok(entity)
    }

    pub async fn update(form: UserForm) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", form.id);
        wr.check()?;
        let entity = Self {
            id: form.id,
            dept_id: form.dept_id,
            username: form.username,
            nick_name: form.nick_name,
            gender: form.gender,
            phone: form.phone,
            email: form.email,
            enabled: form.enabled,
            ..Default::default()
        };
        RB.update_by_wrapper("", &entity, &wr, false).await?;
        Ok(())
    }

    pub async fn delete_batch(ids: Vec<i64>) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.r#in("id", &ids);
        wr.check()?;
        RB.remove_by_wrapper::<Self>("", &wr).await?;

        // 删除用户的角色和岗位
        SysUsersRoles::delete_batch(&ids).await?;
        SysUsersJobs::delete_batch(&ids).await?;
        Ok(())
    }

    pub async fn center(id: i64, form: UpdateCenterForm) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", id);
        wr.check()?;
        let entity = Self {
            nick_name: form.nick_name,
            phone: form.phone,
            gender: form.gender,
            ..Default::default()
        };
        RB.update_by_wrapper("", &entity, &wr, false).await?;
        Ok(())
    }

    pub async fn update_email(id: i64, email: String) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", id);
        wr.check()?;
        let entity = Self {
            email: Option::from(email),
            ..Default::default()
        };
        RB.update_by_wrapper("", &entity, &wr, false).await?;
        Ok(())
    }

    pub async fn update_pass(id: i64, new_pass: String) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", id);
        wr.check()?;
        let entity = Self {
            password: Option::from(new_pass),
            ..Default::default()
        };
        RB.update_by_wrapper("", &entity, &wr, false).await?;
        Ok(())
    }

    // get the user's department
    pub async fn user_dept(user_id: i64) -> Result<Option<UserDeptDto>> {
        let sql = r#"
        SELECT
            sd.dept_id,
            sd.`name`
        FROM
            sys_dept sd,
            sys_user su
        WHERE
            sd.dept_id = su.dept_id
            AND sd.enabled = 1
            AND su.enabled = 1
            AND su.id = #{user_id}
        "#;
        let user_dept: Option<UserDeptDto> =
            RB.py_fetch("", sql, &json!({ "user_id": user_id })).await?;
        Ok(user_dept)
    }

    // get the user's jos
    pub async fn user_jobs(user_id: i64) -> Result<Vec<UserJobDto>> {
        let sql = r#"
        SELECT
            sj.job_id,
            sj.`name` 
        FROM
            sys_job sj,
            sys_users_jobs suj,
            sys_user su 
        WHERE
            sj.job_id = suj.job_id 
            AND suj.user_id = su.id 
            AND sj.enabled = 1 
            AND su.enabled = 1 
            AND suj.user_id = #{user_id}
        ORDER BY
            sj.job_sort
        "#;
        let user_jobs: Vec<UserJobDto> =
            RB.py_fetch("", sql, &json!({ "user_id": user_id })).await?;
        Ok(user_jobs)
    }

    // get the user's role
    pub async fn user_roles(user_id: i64) -> Result<Vec<UserRoleDto>> {
        let sql = r#"
        SELECT
            sr.role_id,
            sr.`name`,
            sr.`level`,
            sr.data_scope 
        FROM
            sys_role sr,
            sys_users_roles sur,
            sys_user su 
        WHERE
            sr.role_id = sur.role_id 
            AND sur.user_id = su.id 
            AND su.enabled = 1 
            AND sur.user_id = #{user_id} 
        ORDER BY
            sr.`level`
        "#;
        let user_roles: Vec<UserRoleDto> =
            RB.py_fetch("", sql, &json!({ "user_id": user_id })).await?;
        Ok(user_roles)
    }

    pub async fn user_data_scopes(user_id: i64) -> Result<HashSet<i64>> {
        let sql = r#"
        SELECT
            srd.dept_id 
        FROM
            sys_roles_depts srd,
            sys_role sr,
            sys_dept sd 
        WHERE
            srd.role_id = sr.role_id 
            AND srd.dept_id = sd.dept_id 
            AND sd.enabled = 1 
            AND sr.role_id IN (
            SELECT
                sur.role_id 
            FROM
                sys_users_roles sur,
                sys_user su,
                sys_role sr 
            WHERE
                sur.user_id = su.id 
                AND sur.role_id = sr.role_id 
            AND su.enabled = 1 
            AND sur.user_id = #{user_id}
           )        
        "#;
        let mut user_role_depts: Vec<i64> =
            RB.py_fetch("", sql, &json!({ "user_id": user_id })).await?;
        let data_scopes =
            user_role_depts
                .iter()
                .fold(HashSet::new(), |mut data_scopes, dept_id| {
                    data_scopes.insert(*dept_id);

                    async {
                        SysDept::lit_children_id_by_pid(*dept_id, &mut data_scopes).await;
                    };
                    data_scopes
                });
        Ok(data_scopes)
    }

    pub async fn user_menu_permissions(user_id: i64) -> Result<HashSet<String>> {
        let sql = r#"
        SELECT DISTINCT
            sm.permission 
        FROM
            sys_roles_menus srm,
            sys_menu sm,
            sys_role sr 
        WHERE
            srm.menu_id = sm.id 
            AND srm.role_id = sr.role_id 
            AND sm.permission IS NOT NULL 
            AND sm.permission != '' 
            AND srm.role_id IN (
            SELECT
                sur.role_id 
            FROM
                sys_users_roles sur,
                sys_user su,
                sys_role sr 
            WHERE
                sur.user_id = su.id 
                AND sur.role_id = sr.role_id 
                AND su.enabled = 1 
            AND sur.user_id = #{user_id}
            )
        ORDER BY
            sm.menu_sort
        "#;
        let mut user_menu_permissions: HashSet<String> =
            RB.py_fetch("", sql, &json!({ "user_id": user_id })).await?;

        Ok(user_menu_permissions)
    }

    pub async fn export_list() -> Result<Vec<UserExportDto>> {
        let sql = r#"
        SELECT
            su.username ,
            r.role,
            sd.name as dept,
            j.job,
            su.email ,
            if(su.enabled = 1,
            '启用',
            '禁用') as enabled ,
            su.phone ,
            su.pwd_reset_time ,
            su.create_time
        FROM
            sys_user su
        left join (
            SELECT
                sur.user_id ,
                GROUP_CONCAT(sr.name) as role
            FROM
                sys_users_roles sur ,
                sys_role sr ,
                sys_user su
            where
                sur.user_id = su.id
                and sr.role_id = sr.role_id
            GROUP by
                sur.user_id )r on
            r.user_id = su.id
        left join sys_dept sd on
            sd.dept_id = su.dept_id
        left join (
            SELECT
                suj.user_id ,
                GROUP_CONCAT(sj.name) as job
            FROM
                sys_users_jobs suj,
                sys_job sj ,
                sys_user su
            where
                suj.job_id = sj.job_id
                and suj.user_id = su.id
            GROUP by
                suj.user_id )j on
            j.user_id = su.id
        "#;
        let export_list: Vec<UserExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
