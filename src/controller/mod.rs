use actix_web::{web, HttpResponse, Result};
use async_trait::async_trait;
use rbatis::plugin::page::{Page, PageRequest};

pub use alipay::AlipayController;
pub use app::AppController;
pub use auth::AuthController;
pub use authorization::AuthorizationController;
pub use column_config::ColumnConfigController;
pub use database::DatabaseController;
pub use deploy::DeployController;
pub use deploy_history::DeployHistoryController;
pub use dept::DeptController;
pub use dict::DictController;
pub use dict_detail::DictDetailController;
pub use email::EmailController;
pub use gen_config::GenConfigController;
pub use generator::GeneratorController;
pub use job::JobController;
pub use limit::LimitController;
pub use local_storage::LocalStorageController;
pub use menu::MenuController;
pub use monitor::MonitorController;
pub use online::OnlineController;
pub use qiniu::QiniuController;
pub use quartz_job::QuartzJobController;
pub use role::RoleController;
pub use sys_log::LogController;
pub use user::UserController;
pub use verify::VerifyController;

use crate::commons::{Resp, RespBuilder, RespErr};
use crate::models::RBatisModel;

mod alipay;
mod app;
mod auth;
mod authorization;
mod column_config;
mod database;
mod deploy;
mod deploy_history;
mod dept;
mod dict;
mod dict_detail;
mod email;
mod gen_config;
mod generator;
mod job;
mod limit;
mod local_storage;
mod menu;
mod monitor;
mod online;
mod qiniu;
mod quartz_job;
mod role;
mod sys_log;
mod user;
mod verify;

#[async_trait]
pub trait Controller: Send + Sync {
    type M: RBatisModel;

    async fn page(pr: PageRequest) -> Result<Resp<Page<Self::M>>, RespErr> {
        let resp_data = Self::M::page(pr).await?;
        Ok(RespBuilder::ok().with_data(resp_data).build())
    }

    async fn list() -> Result<Resp<Vec<Self::M>>, RespErr> {
        let resp_data = Self::M::list().await?;
        Ok(RespBuilder::ok().with_data(resp_data).build())
    }

    async fn get_one(id: web::Path<i64>) -> Result<Resp<Option<Self::M>>, RespErr> {
        let id = id.0;
        let model: Option<Self::M> = Self::M::get_one(id).await?;
        Ok(RespBuilder::ok().with_data(model).build())
    }

    async fn save(form: web::Json<Self::M>) -> Result<Resp<&'static str>, RespErr> {
        let model = form.0;
        Self::M::save(model).await?;
        Ok(RespBuilder::<&str>::ok().build())
    }

    async fn save_batch(form: web::Json<Vec<Self::M>>) -> Result<Resp<&'static str>, RespErr> {
        let models = form.0;
        Self::M::save_batch(&models).await?;
        Ok(RespBuilder::<&str>::ok().build())
    }

    async fn update(form: web::Json<Self::M>) -> Result<Resp<&'static str>, RespErr> {
        let model = form.0;
        Self::M::update(model).await?;
        Ok(RespBuilder::<&str>::ok().build())
    }

    async fn remove_by_id(id: web::Path<i64>) -> Result<Resp<&'static str>, RespErr> {
        Self::M::remove_by_id(id.0).await?;
        Ok(RespBuilder::<&str>::ok().build())
    }

    async fn remove_batch_by_ids(form: web::Json<Vec<i64>>) -> Result<Resp<&'static str>, RespErr> {
        let ids = form.0;
        Self::M::remove_batch_by_ids(&ids).await?;
        Ok(RespBuilder::<&str>::ok().build())
    }

    async fn export_list() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
