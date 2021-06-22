use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::column_config::CodeColumnConfigQuery;
use crate::models::RBatisModel;
use crate::RB;

/// 代码生成字段信息存储
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct CodeColumnConfig {
    /// ID
    pub id: Option<i64>,
    /// 表名
    pub table_name: Option<String>,
    /// 字段名
    pub column_name: Option<String>,
    /// 字段类型
    pub column_type: Option<String>,
    /// 关联字典
    pub dict_name: Option<String>,
    /// 额外信息
    pub extra: Option<String>,
    /// 表单
    pub form_show: Option<u8>,
    /// 表单类型
    pub form_type: Option<String>,
    ///
    pub key_type: Option<String>,
    /// 列表
    pub list_show: Option<u8>,
    /// 必填
    pub not_null: Option<u8>,
    /// 查询方式
    pub query_type: Option<String>,
    /// 字段描述
    pub remark: Option<String>,
    /// 日期注解
    pub date_annotation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableColumn {
    // pub COLUMN_NAME: Option<String>,
    // pub IS_NULLABLE: Option<String>,
    // pub DATA_TYPE: Option<String>,
    // pub COLUMN_COMMENT: Option<String>,
    // pub COLUMN_KEY: Option<String>,
    // pub EXTRA: Option<String>,
    pub column_name: Option<String>,
    pub is_nullable: Option<String>,
    pub data_type: Option<String>,
    pub column_comment: Option<String>,
    pub column_key: Option<String>,
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableInfo {
    pub table_name: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub engine: Option<String>,
    pub table_collation: Option<String>,
    pub table_comment: Option<String>,
}

impl CodeColumnConfig {
    pub async fn tables(query: CodeColumnConfigQuery) -> Result<serde_json::Value> {
        let sql = r#"
        SELECT table_name,`ENGINE`,create_time,table_collation,table_comment FROM information_schema.`TABLES` 
        WHERE TABLE_SCHEMA=(SELECT DATABASE ()) 
        if table_name != null:
          AND INSTR(table_name,#{table_name}) 
        ORDER BY create_time
        "#;

        let res: serde_json::Value = RB
            .py_fetch("", sql, &json!({ "table_name": query.table_name }))
            .await?;
        Ok(res)
    }

    pub async fn list_by_table_name(table_name: &str) -> Result<Vec<Self>> {
        let mut w = RB.new_wrapper();
        if !table_name.is_empty() {
            w.eq("table_name", &table_name);
        }
        w.check()?;

        let list = RB.list_by_wrapper("", &w).await?;

        if !list.is_empty() {
            return Ok(list);
        }

        let sql = r#"
        select
            column_name,
            is_nullable,
            data_type,
            column_comment,
            column_key,
            extra
        from
            information_schema.columns
        where
            table_name = #{table_name}
            and table_schema = (
            select
                database())
        order by
            ordinal_position
        "#;

        let data: Vec<TableColumn> = RB
            .py_fetch("", &sql, &json!({ "table_name": table_name }))
            .await?;

        let entity_list = data.into_iter().fold(Vec::new(), |mut entity_list, v| {
            let ccc = Self {
                table_name: Some(table_name.to_string()),
                column_name: v.column_name,
                column_type: v.data_type,
                extra: v.extra,
                form_show: Some(1),
                key_type: v.column_key,
                list_show: Some(1),
                not_null: Some({
                    if "NO".eq(&v.is_nullable.unwrap()) {
                        1
                    } else {
                        0
                    }
                }),
                remark: v.column_comment,
                ..Default::default()
            };
            entity_list.push(ccc);
            entity_list
        });
        RB.save_batch("", &entity_list).await?;
        Ok(entity_list)
    }

    pub async fn update_batch(form: Vec<Self>) -> Result<()> {
        RB.update_batch_by_id("", &form).await?;
        Ok(())
    }
}
