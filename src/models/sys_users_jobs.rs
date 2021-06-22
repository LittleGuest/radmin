use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 用户岗位关联
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysUsersJobs {
    /// 用户ID
    pub user_id: Option<i64>,
    /// 岗位ID
    pub job_id: Option<i64>,
}

impl SysUsersJobs {
    pub async fn save_batch(entity_list: &[Self]) -> Result<()> {
        RB.save_batch("", entity_list).await?;
        Ok(())
    }

    pub async fn delete_batch(user_ids: &[i64]) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.r#in("user_id", user_ids);
        wr.check()?;
        RB.remove_by_wrapper::<Self>("", &wr).await?;
        Ok(())
    }
}
