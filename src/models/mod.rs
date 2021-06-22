use anyhow::Result;
use async_trait::async_trait;
use rbatis::crud::{CRUD, CRUDEnable};
use rbatis::plugin::page::{Page, PageRequest};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde::export::fmt::Display;

pub use code_column_config::CodeColumnConfig;
pub use code_gen_config::CodeGenConfig;
pub use mnt_app::MntApp;
pub use mnt_database::MntDatabase;
pub use mnt_deploy::MntDeploy;
pub use mnt_deploy_history::MntDeployHistory;
pub use mnt_deploy_server::MntDeployServer;
pub use mnt_server::MntServer;
pub use sys_dept::SysDept;
pub use sys_dict::SysDict;
pub use sys_dict_detail::SysDictDetail;
pub use sys_job::SysJob;
pub use sys_log::SysLog;
pub use sys_menu::SysMenu;
pub use sys_quartz_job::SysQuartzJob;
pub use sys_quartz_log::SysQuartzLog;
pub use sys_role::SysRole;
pub use sys_roles_depts::SysRolesDepts;
pub use sys_roles_menus::SysRolesMenus;
pub use sys_user::SysUser;
pub use sys_users_jobs::SysUsersJobs;
pub use sys_users_roles::SysUsersRoles;
pub use tool_alipay_config::ToolAlipayConfig;
pub use tool_email_config::ToolEmailConfig;
pub use tool_local_storage::ToolLocalStorage;
pub use tool_qiniu_config::ToolQiniuConfig;
pub use tool_qiniu_content::ToolQiniuContent;

use crate::RB;

mod code_column_config;
mod code_gen_config;
mod mnt_app;
mod mnt_database;
mod mnt_deploy;
mod mnt_deploy_history;
mod mnt_deploy_server;
mod mnt_server;
mod sys_dept;
mod sys_dict;
mod sys_dict_detail;
mod sys_job;
mod sys_log;
mod sys_menu;
mod sys_quartz_job;
mod sys_quartz_log;
mod sys_role;
mod sys_roles_depts;
mod sys_roles_menus;
mod sys_user;
mod sys_users_jobs;
mod sys_users_roles;
mod tool_alipay_config;
mod tool_email_config;
mod tool_local_storage;
mod tool_qiniu_config;
mod tool_qiniu_content;

#[async_trait]
pub trait RBatisModel: CRUDEnable + Sized {
    async fn page(pr: PageRequest) -> Result<Page<Self>>;
    async fn list() -> Result<Vec<Self>>;
    async fn get_one(id: i64) -> Result<Option<Self>>;
    async fn save(model: Self) -> Result<Option<i64>>;
    async fn save_batch(models: &[Self]) -> Result<u64>;
    async fn update(model: Self) -> Result<u64>;
    async fn remove_by_id(id: i64) -> Result<u64>;
    async fn remove_batch_by_ids(ids: &[i64]) -> Result<u64>;
    async fn export_list<T>(sql: &str) -> Result<Vec<T>>
        where
            T: DeserializeOwned + Send + Sync,
    {
        let export_list: Vec<T> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}

#[cfg(test)]
mod model_test {
    use anyhow::Result;
    use async_trait::async_trait;
    use rbatis::crud::CRUD;
    use rbatis::plugin::page::{Page, PageRequest};

    use crate::RB;

    use super::RBatisModel;

    #[test]
    pub fn model_test() {}

    #[crud_enable]
    struct Model {
        pub id: Option<i64>,
    }

    #[async_trait]
    impl RBatisModel for Model {
        async fn page(pr: PageRequest) -> Result<Page<Self>> {
            let pages: Page<Self> = RB.fetch_page_by_wrapper("", &RB.new_wrapper(), &pr).await?;
            Ok(pages)
        }

        async fn list() -> Result<Vec<Self>> {
            let list: Vec<Self> = RB.list("").await?;
            Ok(list)
        }

        async fn get_one(id: i64) -> Result<Option<Self>> {
            let mut wr = RB.new_wrapper();
            wr.eq("id", id);
            wr.check()?;

            let model = RB.fetch_by_wrapper("", &wr).await?;
            Ok(model)
        }

        async fn save(model: Self) -> Result<Option<i64>> {
            let dbe_result = RB.save("", &model).await?;
            Ok(dbe_result.last_insert_id)
        }

        async fn save_batch(models: &[Self]) -> Result<u64> {
            let dbe_result = RB.save_batch("", &models).await?;
            Ok(dbe_result.rows_affected)
        }

        async fn update(model: Self) -> Result<u64> {
            let affected_rows = RB.update_by_id("", &model).await?;
            Ok(affected_rows)
        }

        async fn remove_by_id(id: i64) -> Result<u64> {
            let affected_rows = RB.remove_by_id::<Self>("", &id).await?;
            Ok(affected_rows)
        }

        async fn remove_batch_by_ids(ids: &[i64]) -> Result<u64> {
            let affected_rows = RB.remove_batch_by_id::<Self>("", ids).await?;
            Ok(affected_rows)
        }
    }
}
