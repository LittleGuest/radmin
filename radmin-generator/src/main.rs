#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use anyhow::Result;
use chrono::NaiveDateTime;
use fast_log::fast_log;
use heck::CamelCase;
use log::Level;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};

use common::*;
use convert::*;
use model::*;

mod common;
mod convert;
mod model;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref KEYWORDS: Vec<&'static str> = {
        vec![
            "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
            "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
            "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "abstract", "async", "await", "become", "box", "do",
            "use", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
            "yield",
        ]
    };
}

#[async_std::main]
async fn main() -> Result<()> {
    RB.link(
        format!(
            "{}://{}:{}@{}:{}/{}",
            DRIVER, USERNAME, PASSWORD, HOST, PORT, DATABASE
        )
            .as_str(),
    )
        .await?;

    generator("../tps/", &[]).await?;

    Ok(())
}

async fn generator(path: &str, table_names: &[&str]) -> Result<()> {
    println!("====== the game has started ======");

    let table_names_string = table_names.join(",");
    let tables = tables(&table_names_string).await?;
    let tables_columns = tables_columns(&table_names_string).await?;
    if tables.is_empty() || tables_columns.is_empty() {
        return Ok(());
    }

    let table_map: HashMap<String, Table> = tables
        .into_iter()
        .map(|t| (t.table_name.clone().unwrap(), t))
        .collect();

    let table_column_map =
        table_map
            .keys()
            .fold(HashMap::new(), |mut table_column_map, table_name| {
                table_column_map.insert(
                    table_name,
                    tables_columns
                        .iter()
                        .filter(|table_column| Some(table_name.clone()) == table_column.table_name)
                        .collect::<Vec<_>>(),
                );
                table_column_map
            });

    fs::create_dir_all(path)?;
    let mut tera = tera::Tera::default();
    table_map.iter().for_each(|(table_name, table)| {
        let column = table_column_map.get(&table_name);

        let mut ctx = tera::Context::new();
        ctx.insert("struct_name", &table_name.to_camel_case());
        ctx.insert("table", &table);
        let mut has_columns = false;
        if let Some(columns) = column {
            has_columns = !columns.is_empty();

            let cols = columns.iter().fold(Vec::new(), |mut cols, column| {
                let mut tc = TableColumn {
                    table_schema: column.table_schema.clone(),
                    table_name: column.table_name.clone(),
                    column_name: {
                        let column_name = column.column_name.clone().unwrap();
                        if KEYWORDS.contains(&column_name.as_str()) {
                            Some(format!("r#{}", column_name))
                        } else {
                            Some(column_name)
                        }
                    },
                    ordinal_position: column.ordinal_position,
                    column_default: column.column_default.clone(),
                    is_nullable: column.is_nullable.clone(),
                    data_type: column.data_type.clone(),
                    character_maximum_length: column.character_maximum_length,
                    column_type: column.column_type.clone(),
                    column_key: column.column_key.clone(),
                    column_comment: column.column_comment.clone(),
                    field_type: Some(mysql_2_rust(
                        &column.data_type.clone().unwrap_or_default().to_uppercase(),
                    )),
                    multi_world: Some({
                        column
                            .column_name
                            .clone()
                            .unwrap()
                            .contains(|c| c == '_' || c == '-')
                    }),
                };
                cols.push(tc);
                cols
            });
            ctx.insert("columns", &cols);
        }
        ctx.insert("has_columns", &has_columns);
        let render_string = tera.render_str(MODEL_TEMPLATE, &ctx).expect("渲染模板错误");

        let model_file_name = format!("{}{}.rs", path, &table_name);
        let mut tf = fs::File::create(&model_file_name).expect("创建文件失败");
        tf.write_all(render_string.as_bytes())
            .expect("写入数据错误");

        println!("the {} has been generated", &model_file_name);
    });

    let mut ctx = tera::Context::new();
    ctx.insert("table_names", &table_map);
    let render_string = tera.render_str(MOD_TEMPLATE, &ctx)?;

    let mod_file_name = format!("{}mod.rs", path);
    let mut tf = fs::File::create(&mod_file_name)?;
    tf.write_all(render_string.as_bytes())?;

    println!("the {} has been generated", &mod_file_name);
    println!("====== the game is over ======");
    Ok(())
}
