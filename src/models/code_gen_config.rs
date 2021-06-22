use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 代码生成器配置
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct CodeGenConfig {
    /// ID
    pub id: Option<i64>,
    /// 表名
    pub table_name: Option<String>,
    /// 作者
    pub author: Option<String>,
    /// 是否覆盖
    pub cover: Option<u8>,
    /// 模块名称
    pub module_name: Option<String>,
    /// 至于哪个包下
    pub pack: Option<String>,
    /// 前端代码生成的路径
    pub path: Option<String>,
    /// 前端Api文件路径
    pub api_path: Option<String>,
    /// 表前缀
    pub prefix: Option<String>,
    /// 接口名称
    pub api_alias: Option<String>,
}

impl CodeGenConfig {
    pub async fn get_by_table_name(table_name: &str) -> Result<Option<Self>> {
        let mut w = RB.new_wrapper();
        w.eq("table_name", table_name);
        w.check()?;
        let gen_config: Self = RB.fetch_by_wrapper("", &w).await?;
        Ok(Option::from(gen_config))
    }

    pub async fn update(form: Self) -> Result<()> {
        RB.update_by_id("", &form).await?;
        Ok(())
    }
}
