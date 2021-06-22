use std::collections::HashMap;

use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::dict::{DictDetailDto, SysDictDetailQuery};
use crate::models::RBatisModel;
use crate::models::SysDict;
use crate::RB;

/// 数据字典详情
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysDictDetail {
    /// ID
    pub id: Option<i64>,
    /// 字典id
    pub dict_id: Option<i64>,
    /// 字典标签
    pub label: Option<String>,
    /// 字典值
    pub value: Option<String>,
    /// 排序
    pub dict_sort: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysDictDetail {
    pub async fn page(query: SysDictDetailQuery) -> Result<Page<Self>> {
        let sql = r#"
        SELECT
            sdd.id ,
            sdd.dict_id ,
            sdd.label ,
            sdd.value ,
            sdd.dict_sort
        FROM
            sys_dict_detail sdd ,
            sys_dict sd
        where
            sdd.dict_id = sd.id
            if dict_name != null:
              and INSTR(sd.name, #{dict_name})
            if label != null:
              and INSTR(sdd.label, #{label})
        ORDER BY
            sdd.dict_sort
        "#;

        let pages = RB
            .py_fetch_page(
                "",
                &sql,
                &json!({ "dict_name": query.dict_name, "label": query.label }),
                &PageRequest::new(query.current.unwrap_or(1), query.size.unwrap_or(20)),
            )
            .await?;

        Ok(pages)
    }

    pub async fn maps(dict_names: Vec<&str>) -> Result<HashMap<String, Vec<DictDetailDto>>> {
        let sql = r#"
        SELECT
            sdd.id ,
            sdd.dict_id ,
            sdd.label ,
            sdd.value ,
            sdd.dict_sort
        FROM
            sys_dict_detail sdd ,
            sys_dict sd
        where
            sdd.dict_id = sd.id
            and sd.name IN (
            trim ',':
                for item in names:
                  #{item},
            )
        "#;
        let dtos: Vec<DictDetailDto> = RB
            .py_fetch("", &sql, &json!({ "names": dict_names }))
            .await?;

        let map = dict_names
            .iter()
            .fold(HashMap::new(), |mut map, dict_name| {
                async {
                    let dict = SysDict::get_dict_by_name(dict_name.to_string())
                        .await
                        .unwrap_or_default();

                    if let Some(dict) = dict {
                        let temp = dtos
                            .iter()
                            .filter(|dto| dict.id.eq(&dto.dict_id))
                            .cloned()
                            .collect::<Vec<_>>();
                        map.insert(dict_name.to_string(), temp);
                    }
                };
                map
            });

        Ok(map)
    }
}
