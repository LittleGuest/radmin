use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 角色菜单关联
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysRolesMenus {
    /// 菜单ID
    pub menu_id: Option<i64>,
    /// 角色ID
    pub role_id: Option<i64>,
}

impl SysRolesMenus {
    pub async fn save_batch(entity_list: &[Self]) -> Result<()> {
        RB.save_batch("", entity_list).await?;
        Ok(())
    }

    pub async fn delete_batch_by_role(role_id: i64) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.eq("role_id", role_id);
        wr.check()?;
        RB.remove_by_wrapper::<Self>("", &wr).await?;
        Ok(())
    }

    pub async fn roles_menus(role_ids: &[i64]) -> Result<Vec<i64>> {
        let mut wr = RB.new_wrapper();
        wr.r#in("role_id", role_ids);
        wr.check()?;

        let roles_menus = RB.list_by_wrapper::<Self>("", &wr).await?;
        let menu_ids: Vec<i64> = roles_menus.iter().map(|rm| rm.menu_id.unwrap()).collect();
        Ok(menu_ids)
    }
}
