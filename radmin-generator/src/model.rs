use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::RB;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub(crate) struct Table {
    // pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    // pub table_type: Option<String>,
    // pub engine: Option<String>,
    // pub version: Option<String>,
    // pub row_format: Option<String>,
    // pub table_rows: Option<String>,
    // pub avg_row_length: Option<String>,
    // pub data_length: Option<String>,
    // pub max_data_length: Option<String>,
    // pub index_length: Option<String>,
    // pub data_free: Option<String>,
    // pub auto_increment: Option<String>,
    // pub create_time: Option<NaiveDateTime>,
    // pub update_time: Option<NaiveDateTime>,
    // pub check_time: Option<NaiveDateTime>,
    // pub table_collation: Option<String>,
    // pub checksum: Option<String>,
    // pub create_options: Option<String>,
    pub table_comment: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub(crate) struct TableColumn {
    // pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    pub column_name: Option<String>,
    pub ordinal_position: Option<u8>,
    pub column_default: Option<String>,
    pub is_nullable: Option<String>,
    pub data_type: Option<String>,
    pub character_maximum_length: Option<u64>,
    // pub character_octet_length: Option<String>,
    // pub numeric_precision: Option<String>,
    // pub numeric_scale: Option<String>,
    // pub datetime_precision: Option<String>,
    // pub character_set_name: Option<String>,
    // pub collation_name: Option<String>,
    pub column_type: Option<String>,
    pub column_key: Option<String>,
    // pub extra: Option<String>,
    // pub privileges: Option<String>,
    pub column_comment: Option<String>,
    // pub generation_expression: Option<String>,
    // pub srs_id: Option<String>,

    // 对应 Rust 类型
    pub field_type: Option<String>,
    pub multi_world: Option<bool>,
}

pub(crate) async fn tables(table_names: &str) -> Result<Vec<Table>> {
    let sql = r#"
    SELECT
        TABLE_CATALOG,
        TABLE_SCHEMA,
        TABLE_NAME,
        TABLE_TYPE,
        `ENGINE`,
        VERSION,
        ROW_FORMAT,
        TABLE_ROWS,
        AVG_ROW_LENGTH,
        DATA_LENGTH,
        MAX_DATA_LENGTH,
        INDEX_LENGTH,
        DATA_FREE,
        AUTO_INCREMENT,
        CREATE_TIME,
        UPDATE_TIME,
        CHECK_TIME,
        TABLE_COLLATION,
        `CHECKSUM`,
        CREATE_OPTIONS,
        TABLE_COMMENT
    FROM
        information_schema.`TABLES`
    WHERE
        TABLE_SCHEMA = (
        SELECT
            DATABASE ())
        if table_name != null:
          AND INSTR(TABLE_NAME, #{table_name})
    ORDER BY
        CREATE_TIME
    "#;

    let res: Vec<Table> = RB
        .py_fetch("", sql, &json!({ "table_name": table_names }))
        .await?;
    Ok(res)
}

pub(crate) async fn tables_columns(table_names: &str) -> Result<Vec<TableColumn>> {
    let sql = r#"
    SELECT
        TABLE_CATALOG,
        TABLE_SCHEMA,
        TABLE_NAME,
        COLUMN_NAME,
        ORDINAL_POSITION,
        COLUMN_DEFAULT,
        IS_NULLABLE,
        DATA_TYPE,
        CHARACTER_MAXIMUM_LENGTH,
        CHARACTER_OCTET_LENGTH,
        NUMERIC_PRECISION,
        NUMERIC_SCALE,
        DATETIME_PRECISION,
        CHARACTER_SET_NAME,
        COLLATION_NAME,
        COLUMN_TYPE,
        COLUMN_KEY,
        EXTRA,
        `PRIVILEGES`,
        COLUMN_COMMENT,
        GENERATION_EXPRESSION,
        SRS_ID
    FROM
        information_schema.COLUMNS
    WHERE
        TABLE_SCHEMA = (
        SELECT
            DATABASE ())
        AND INSTR(TABLE_NAME, #{table_name})
    ORDER BY
        ORDINAL_POSITION
    "#;

    let columns: Vec<TableColumn> = RB
        .py_fetch("", &sql, &json!({ "table_name": table_names }))
        .await?;
    Ok(columns)
}
