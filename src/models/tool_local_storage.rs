use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::local_storage::ToolLocalStorageQuery;
use crate::models::RBatisModel;
use crate::RB;

/// 本地存储
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct ToolLocalStorage {
    /// ID
    pub id: Option<i64>,
    /// 文件真实的名称
    pub real_name: Option<String>,
    /// 文件名
    pub name: Option<String>,
    /// 后缀
    pub suffix: Option<String>,
    /// 路径
    pub path: Option<String>,
    /// 类型
    pub r#type: Option<String>,
    /// 大小
    pub size: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl ToolLocalStorage {
    pub async fn page(query: ToolLocalStorageQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.blurry.is_some() {
            wr.eq("name", &query.blurry);
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

    pub async fn save(entity: Self) -> Result<()> {
        RB.save("", &entity).await?;
        Ok(())
    }

    pub async fn delete_batch_by_ids(ids: Vec<i64>) -> Result<()> {
        let mut wr = RB.new_wrapper();
        wr.r#in("storage_id", &ids);
        wr.check()?;
        RB.remove_by_wrapper::<Self>("", &wr).await?;
        Ok(())
    }
}
