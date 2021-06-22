use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 角色部门关联
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysRolesDepts {
    /// 角色ID
    pub role_id: Option<i64>,
    /// 部门ID
    pub dept_id: Option<i64>,
}

impl SysRolesDepts {
    pub async fn save_batch(entity_list: &[Self]) -> Result<()> {
        RB.save_batch("", entity_list).await?;
        Ok(())
    }
}
