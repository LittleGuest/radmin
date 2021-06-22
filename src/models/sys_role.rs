use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::role::{RoleExportDto, RoleForm, RoleQuery};
use crate::models::RBatisModel;
use crate::models::SysRolesDepts;
use crate::RB;

/// 角色表
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysRole {
    /// ID
    pub id: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 角色级别
    pub level: Option<i32>,
    /// 描述
    pub description: Option<String>,
    /// 数据权限
    pub data_scope: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysRole {
    pub async fn page(query: RoleQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.blurry.is_some() {
            wr.eq("name", query.blurry);
        }
        wr.order_by(true, &["level"]);
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

    pub async fn save(form: RoleForm) -> Result<Self> {
        let entity = Self {
            name: form.name,
            level: form.level,
            description: form.description,
            data_scope: form.data_scope,
            ..Default::default()
        };

        let dbr = RB.save("", &entity).await?;

        // 角色ID
        if let Some(dept_ids) = form.depts {
            let srds = dept_ids.iter().fold(Vec::new(), |mut srds, dept_id| {
                let srd = SysRolesDepts {
                    role_id: dbr.last_insert_id,
                    dept_id: Option::from(*dept_id),
                };
                srds.push(srd);
                srds
            });
            SysRolesDepts::save_batch(&srds).await;
        }

        Ok(entity)
    }

    pub async fn update(form: RoleForm) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", form.id);
        wr.check()?;

        let entity = Self {
            name: form.name,
            level: form.level,
            description: form.description,
            data_scope: form.data_scope,
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
        Ok(())
    }

    pub async fn get_by_id(id: i64) -> Result<Self> {
        let mut wr = RB.new_wrapper();
        wr.eq("id", id);
        wr.check()?;
        let res: Self = RB.fetch_by_wrapper("", &wr).await?;
        Ok(res)
    }

    pub async fn all() -> Result<Vec<Self>> {
        let mut wr = RB.new_wrapper();
        wr.order_by(true, &["level"]);
        wr.check()?;
        let all: Vec<Self> = RB.list_by_wrapper("", &wr).await?;
        Ok(all)
    }

    pub async fn level(id: i64) -> Result<Vec<i64>> {
        let sql = r#"
        SELECT
            sr.`level`
        FROM
            sys_users_roles sur,
            sys_user su ,
            sys_role sr
        where
            su.user_id = sur.user_id
            and sr.id = sur.role_id
            and sur.user_id = #{user_id}
        "#;
        let levels: Vec<i64> = RB.py_fetch("", sql, &json!({ "user_id": id })).await?;
        Ok(levels)
    }

    pub async fn export_list() -> Result<Vec<RoleExportDto>> {
        let sql = r#"
        SELECT
            sr.name ,
            sr.`level` ,
            sr.description ,
            sr.create_time
        FROM
            sys_role sr
        ORDER BY sr.level
        "#;
        let export_list: Vec<RoleExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
