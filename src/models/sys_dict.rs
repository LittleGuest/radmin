use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::dict::{DictDetailExportDto, SysDictQuery};
use crate::models::RBatisModel;
use crate::models::SysDictDetail;
use crate::RB;

/// 数据字典
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysDict {
    /// ID
    pub id: Option<i64>,
    /// 字典名称
    pub name: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysDict {
    pub async fn page(query: SysDictQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.blurry.is_some() {
            wr.like("name", &query.blurry)
                .or()
                .like("description", &query.blurry);
        }
        // if query.sort.is_some() {
        //     wr.order_by(
        //         query.is_asc.unwrap_or(false),
        //         &[query.sort.unwrap_or("id".to_string()).as_str()],
        //     );
        // }
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

    pub async fn delete_batch(ids: Vec<i64>) -> Result<()> {
        let _ = RB.remove_batch_by_id::<Self>("", &ids).await?;

        let mut wr = RB.new_wrapper();
        wr.r#in("dict_id", &ids);
        wr.check()?;
        RB.remove_by_wrapper::<SysDictDetail>("", &wr).await?;
        Ok(())
    }

    pub async fn get_dict_by_name(name: String) -> Result<Option<Self>> {
        let mut wr = RB.new_wrapper();
        wr.eq("name", name);
        wr.check()?;
        let res: Self = RB.fetch_by_wrapper("", &wr).await?;
        Ok(Option::from(res))
    }

    pub async fn export_list() -> Result<Vec<DictDetailExportDto>> {
        let sql = r#"
        SELECT
            sd.name,
            sd.description ,
            sdd.label ,
            sdd.value ,
            sdd.create_time
        FROM
            sys_dict_detail sdd ,
            sys_dict sd
        where
            sdd.dict_id = sd.id
        ORDER by
            sdd.dict_sort
        "#;
        let export_list: Vec<DictDetailExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
