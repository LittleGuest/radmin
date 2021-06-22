use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use actix_web::{HttpResponse, Responder, web};
use serde_json::Value;

use crate::{MYSQL_TO_JAVA, TEMPLATE_FILE_SUFFIX, TEMPLATE_NAMES, TEMPLATES};
use crate::commons::RespBuilder;
use crate::dtos::column_config::CodeColumnConfigQuery;
use crate::models::{CodeColumnConfig, CodeGenConfig};

pub struct ColumnConfigController;

impl ColumnConfigController {
    pub async fn tables(query: web::Query<CodeColumnConfigQuery>) -> impl Responder {
        RespBuilder::ok()
            .with_data(CodeColumnConfig::tables(query.0).await.unwrap_or_default())
            .build()
    }

    pub async fn columns(query: web::Query<CodeColumnConfigQuery>) -> impl Responder {
        let table_name = query.0.table_name.unwrap_or_default();
        RespBuilder::ok()
            .with_data(
                CodeColumnConfig::list_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default(),
            )
            .build()
    }

    pub async fn sync(_form: web::Json<Vec<String>>) -> impl Responder {
        // TODO
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update_batch(form: web::Json<Vec<CodeColumnConfig>>) -> impl Responder {
        CodeColumnConfig::update_batch(form.0)
            .await
            .unwrap_or_default();
        RespBuilder::<&str>::ok().build()
    }

    pub async fn generator(
        language: web::Path<String>,
        web::Path((table_name, event_type)): web::Path<(String, u32)>,
    ) -> HttpResponse {
        let mut language = language.0;
        if language.is_empty() {
            language = "rust".to_string();
        }
        // TODO 生产环境关闭该功能
        return match event_type {
            // 生成代码
            0 => {
                let code_gen_config = CodeGenConfig::get_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                let code_column_config = CodeColumnConfig::list_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                Self::generate_code(language, code_gen_config, code_column_config);
                HttpResponse::Ok().json(())
            }
            // 预览
            1 => {
                let code_gen_config = CodeGenConfig::get_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                let code_column_config = CodeColumnConfig::list_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                Self::preview_code(language, code_gen_config, code_column_config)
            }
            // 打包下载
            2 => {
                let code_gen_config = CodeGenConfig::get_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                let code_column_config = CodeColumnConfig::list_by_table_name(table_name.as_str())
                    .await
                    .unwrap_or_default();
                Self::download_code(language, code_gen_config, code_column_config)
            }
            _ => HttpResponse::Ok().json(RespBuilder::<&str>::ok().with_msg("无此项").build()),
        };
    }

    // 生成代码
    fn generate_code(
        language: String,
        code_gen_config: Option<CodeGenConfig>,
        code_column_config: Vec<CodeColumnConfig>,
    ) {
        if code_gen_config.is_none() {
            return;
        }

        let code_gen_config = code_gen_config.unwrap_or_default();
        let mut ctx = tera::Context::new();
        let gen_map = Self::get_gen_map(&language, &code_gen_config, &code_column_config);
        ctx.insert("data", &gen_map);

        let table_name = code_gen_config.table_name.unwrap();
        let suffix = TEMPLATE_FILE_SUFFIX.get(language.as_str()).unwrap();

        TEMPLATE_NAMES
            .get(language.as_str())
            .unwrap()
            .iter()
            .for_each(|template_name| {
                let body = TEMPLATES.render(
                    format!("{}{}{}.html", language, "/back/", *template_name).as_str(),
                    &ctx,
                );
                if let Ok(r) = body {
                    let _ = std::fs::create_dir_all(format!("./tps/{}/", language));

                    let template_file = File::create(format!("./tps/{}.{}", table_name, *suffix));
                    if let Ok(mut tf) = template_file {
                        let _ = tf.write_all(r.as_bytes());
                    } else {
                        println!("创建{}模板文件失败", *template_name);
                    }
                } else {
                    println!("渲染{}模板文件失败", *template_name);
                }
            });
    }

    // 预览代码
    fn preview_code(
        language: String,
        code_gen_config: Option<CodeGenConfig>,
        code_column_config: Vec<CodeColumnConfig>,
    ) -> HttpResponse {
        if code_gen_config.is_none() {
            return HttpResponse::Ok()
                .json(RespBuilder::<&str>::ok().with_msg("请先配置生成器").build());
        }

        let code_gen_config = code_gen_config.unwrap_or_default();
        let mut ctx = tera::Context::new();
        let gen_map = Self::get_gen_map(&language, &code_gen_config, &code_column_config);
        ctx.insert("data", &gen_map);

        let preview_codes = TEMPLATE_NAMES.get(&language.as_str()).unwrap().iter().fold(
            Vec::new(),
            |mut preview_codes, template_name| {
                let ts = TEMPLATES.render(
                    format!("{}{}{}.html", language, "/back/", template_name).as_str(),
                    &ctx,
                );
                if let Ok(r) = ts {
                    let mut t_map: HashMap<&str, String> = HashMap::new();
                    t_map.insert(template_name, r);
                    preview_codes.push(t_map);
                } else {
                    println!("渲染{}模板文件失败", template_name);
                }
                preview_codes
            },
        );

        HttpResponse::Ok().json(RespBuilder::ok().with_data(preview_codes).build())
    }

    // 打包下载代码
    fn download_code(
        language: String,
        code_gen_config: Option<CodeGenConfig>,
        code_column_config: Vec<CodeColumnConfig>,
    ) -> HttpResponse {
        if code_gen_config.is_none() {
            return HttpResponse::Ok()
                .json(RespBuilder::<&str>::ok().with_msg("请先配置生成器").build());
        }

        let code_gen_config = code_gen_config.unwrap_or_default();
        let mut ctx = tera::Context::new();
        let gen_map = Self::get_gen_map(&language, &code_gen_config, &code_column_config);
        ctx.insert("data", &gen_map);

        let zip_file = File::create("./tps/code.zip");
        let table_name = code_gen_config.table_name.unwrap();
        if let Ok(zf) = zip_file {
            let mut zw = zip::write::ZipWriter::new(zf);
            TEMPLATE_NAMES
                .get(&language.as_str())
                .unwrap()
                .iter()
                .for_each(|template_name| {
                    let body = TEMPLATES.render(
                        format!("{}{}{}.html", language, "/back/", *template_name).as_str(),
                        &ctx,
                    );
                    if let Ok(r) = body {
                        let _ = zw.start_file(
                            format!("{}.{}", table_name, language),
                            zip::write::FileOptions::default(),
                        );
                        let _ = zw.write_all(r.as_bytes());
                    } else {
                        println!("渲染{}模板文件失败", *template_name);
                    }
                });

            let _ = zw.finish();

            // TODO 下载-文件流
            return HttpResponse::Ok()
                .header("Content-Disposition", "attachment;filename=code.zip")
                .content_type("application/octet-stream")
                .body("");
        }
        HttpResponse::Ok().json(RespBuilder::<&str>::ok().with_msg("下载失败！！！").build())
    }

    // 获取模板数据
    fn get_gen_map(
        _language: &str,
        code_gen_config: &CodeGenConfig,
        code_column_config: &[CodeColumnConfig],
    ) -> HashMap<String, Value> {
        let mut gen_map: HashMap<String, Value> = HashMap::new();

        // 接口别名
        gen_map.insert(
            "api_alias".to_string(),
            Value::from(code_gen_config.api_alias.clone().unwrap()),
        );
        // 包名称
        gen_map.insert(
            "package".to_string(),
            Value::from(code_gen_config.pack.clone().unwrap()),
        );
        // 模块名称
        gen_map.insert(
            "module_name".to_string(),
            Value::from(code_gen_config.module_name.clone().unwrap()),
        );
        // 作者
        gen_map.insert(
            "author".to_string(),
            Value::from(code_gen_config.author.clone().unwrap()),
        );

        // 创建日期
        gen_map.insert(
            "date".to_string(),
            Value::String(
                chrono::Local::today().to_string()
                // .format("%Y-%m-%d_%H:%M:%S")
                // .to_string(),
            ),
        );
        // 表名
        gen_map.insert(
            "table_name".to_string(),
            Value::from(code_gen_config.table_name.clone().unwrap()),
        );

        // 大写开头的类名
        let mut class_name = "".to_string();
        // 小写开头的类名
        let mut change_class_name = "".to_string();
        // 判断是否去除表前缀
        if let Some(p) = &code_gen_config.prefix {
            if !p.is_empty() {
                if code_gen_config
                    .table_name
                    .as_ref()
                    .unwrap()
                    .starts_with(&p.as_str())
                {
                    if let Some(r) = code_gen_config
                        .table_name
                        .as_ref()
                        .unwrap()
                        .strip_prefix(&p.as_str())
                    {
                        class_name = r.to_string();
                    } else {
                        class_name = code_gen_config
                            .table_name
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .to_uppercase();
                    }

                    if let Some(r) = code_gen_config
                        .table_name
                        .clone()
                        .unwrap()
                        .strip_prefix(&p.as_str())
                    {
                        change_class_name = r.to_string();
                    } else {
                        change_class_name = code_gen_config
                            .table_name
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .to_lowercase()
                    }
                } else {
                    class_name = code_gen_config.table_name.clone().unwrap();
                    change_class_name = code_gen_config.table_name.clone().unwrap();
                }
            }
        } else {
            class_name = code_gen_config
                .table_name
                .as_ref()
                .unwrap()
                .as_str()
                .to_uppercase();
            change_class_name = code_gen_config
                .table_name
                .as_ref()
                .unwrap()
                .as_str()
                .to_lowercase();
        }
        // 保存类名
        gen_map.insert("class_name".to_string(), Value::from(class_name));
        // 保存小写开头的类名
        gen_map.insert(
            "change_class_name".to_string(),
            Value::from(change_class_name),
        );
        // 存在 Timestamp 字段
        gen_map.insert("has_timestamp".to_string(), Value::Bool(false));
        // 查询类中存在 Timestamp 字段
        gen_map.insert("query_has_timestamp".to_string(), Value::Bool(false));
        // 存在 BigDecimal 字段
        gen_map.insert("has_big_decimal".to_string(), Value::Bool(false));
        // 查询类中存在 BigDecimal 字段
        gen_map.insert("query_has_big_decimal".to_string(), Value::Bool(false));
        // 是否需要创建查询
        gen_map.insert("has_query".to_string(), Value::Bool(false));
        // 自增主键
        gen_map.insert("auto".to_string(), Value::Bool(false));
        // 存在字典
        gen_map.insert("has_dict".to_string(), Value::Bool(false));
        // 存在日期注解
        gen_map.insert("has_date_annotation".to_string(), Value::Bool(false));

        // 保存字段信息
        let mut columns: Vec<serde_json::Map<String, Value>> = Vec::new();
        // 保存查询字段的信息
        let mut query_columns: Vec<serde_json::Map<String, Value>> = Vec::new();
        // 存储字典信息
        let mut dicts: Vec<String> = Vec::new();
        // 存储 between 信息
        let mut betweens: Vec<serde_json::Map<String, Value>> = Vec::new();
        // 存储不为空的字段信息
        let mut is_not_null_columns: Vec<serde_json::Map<String, Value>> = Vec::new();

        code_column_config.iter().for_each(|column| {
            let mut list_map = serde_json::Map::new();
            // 字段描述
            list_map.insert(
                "remark".to_string(),
                Value::from(column.remark.clone().unwrap()),
            );
            // // 字段类型
            list_map.insert(
                "column_key".to_string(),
                Value::from(column.key_type.clone().unwrap()),
            );
            // 主键类型
            let col_type = MYSQL_TO_JAVA
                .get(column.column_type.clone().unwrap().as_str())
                .unwrap_or(&"");
            // 小写开头的字段名
            let change_column_name = column.column_name.clone().unwrap().as_str().to_lowercase();
            // 大写开头的字段名
            let capital_column_name = column.column_name.clone().unwrap().as_str().to_uppercase();
            if "PRI".eq(column.key_type.as_ref().unwrap()) {
                // 存储主键类型
                gen_map.insert("pk_column_type".to_string(), Value::from(*col_type));
                // 存储小写开头的字段名
                gen_map.insert(
                    "pk_change_column_name".to_string(),
                    Value::from(change_column_name.clone()),
                );
                // 存储大写开头的字段名
                gen_map.insert(
                    "pk_capital_column_name".to_string(),
                    Value::from(capital_column_name.clone()),
                );
            }
            // 是否存在 Timestamp 类型的字段
            if "Timestamp".eq(*col_type) {
                gen_map.insert("has_timestamp".to_string(), Value::Bool(true));
            }
            // 是否存在 BigDecimal 类型的字段
            if "BigDecimal".eq(*col_type) {
                gen_map.insert("has_big_decimal".to_string(), Value::Bool(true));
            }
            // 主键是否自增
            if "auto_increment".eq(column.extra.as_ref().unwrap()) {
                gen_map.insert("auto".to_string(), Value::Bool(true));
            }
            // 主键存在字典
            if let Some(dn) = &column.dict_name {
                if !dn.is_empty() {
                    gen_map.insert("has_dict".to_string(), Value::Bool(true));
                    dicts.push(dn.to_string());
                }
            }
            // 存储字段类型
            list_map.insert("column_type".to_string(), Value::from(*col_type));
            // 存储字原始段名称
            list_map.insert(
                "column_name".to_string(),
                Value::from(column.column_name.clone().unwrap()),
            );
            // 不为空
            list_map.insert(
                "is_not_null".to_string(),
                Value::from(column.not_null.clone().unwrap()),
            );
            // 字段列表显示
            list_map.insert(
                "column_show".to_string(),
                Value::from(column.list_show.clone().unwrap()),
            );
            // 表单显示
            list_map.insert(
                "form_show".to_string(),
                Value::from(column.form_show.clone().unwrap()),
            );
            // 表单组件类型
            list_map.insert(
                "form_type".to_string(),
                Value::from({
                    if let Some(ft) = &column.form_type {
                        if ft.is_empty() {
                            "Input".to_string()
                        } else {
                            ft.to_string()
                        }
                    } else {
                        "Input".to_string()
                    }
                }),
            );
            // 小写开头的字段名称
            list_map.insert(
                "change_column_name".to_string(),
                Value::from(change_column_name.clone()),
            );
            // 大写开头的字段名称
            list_map.insert(
                "capital_column_name".to_string(),
                Value::from(capital_column_name.clone()),
            );
            // 字典名称
            list_map.insert(
                "dict_name".to_string(),
                Value::from(column.dict_name.clone().unwrap_or_default()),
            );
            // 日期注解
            list_map.insert(
                "date_annotation".to_string(),
                Value::from(column.date_annotation.clone().unwrap_or_default()),
            );
            if let Some(da) = &column.date_annotation {
                if !da.is_empty() {
                    gen_map.insert("has_date_annotation".to_string(), Value::Bool(true));
                }
            }
            // 添加非空字段信息
            if let Some(nn) = column.not_null {
                if nn == 1 {
                    is_not_null_columns.push(list_map.clone());
                }
            }
            // 判断是否有查询，如有则把查询的字段set进columnQuery
            if let Some(qt) = &column.query_type {
                if !qt.is_empty() {
                    // 查询类型
                    list_map.insert(
                        "query_type".to_string(),
                        Value::from(column.query_type.clone().unwrap()),
                    );
                    // 是否存在查询
                    gen_map.insert("has_query".to_string(), Value::Bool(true));
                    if "Timestamp".eq(*col_type) {
                        gen_map.insert("query_has_timestamp".to_string(), Value::Bool(true));
                    }
                    if "BigDecimal".eq(*col_type) {
                        gen_map.insert("query_has_big_decimal".to_string(), Value::Bool(true));
                    }
                    let temp_list_map = list_map.clone();
                    if "between".eq_ignore_ascii_case(column.query_type.as_ref().unwrap()) {
                        betweens.push(temp_list_map);
                    } else {
                        // 添加到查询列表中
                        query_columns.push(temp_list_map);
                    }
                }
            }
            // 添加到字段列表中
            columns.push(list_map.clone());
        });

        // 保存字段列表
        gen_map.insert("columns".to_string(), Value::from(columns));
        // 保存查询列表
        gen_map.insert("query_columns".to_string(), Value::from(query_columns));
        // 保存字段列表
        gen_map.insert("dicts".to_string(), Value::from(dicts));
        // 保存查询列表
        gen_map.insert("betweens".to_string(), Value::from(betweens));
        // 保存非空字段信息
        gen_map.insert(
            "is_not_null_columns".to_string(),
            Value::from(is_not_null_columns),
        );
        gen_map
    }
}
