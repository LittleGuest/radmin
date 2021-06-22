use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::controller::Controller;
use crate::dtos::dict::SysDictQuery;
use crate::models::SysDict;

pub struct DictController;

impl Controller for DictController {
    type M = SysDict;
}

impl DictController {
    pub async fn page(query: web::Query<SysDictQuery>) -> impl Responder {
        let data = SysDict::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn delete_batch(ids: web::Json<Vec<i64>>) -> impl Responder {
        let ids = ids.0;
        SysDict::delete_batch(ids).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn export() -> impl Responder {
        let export_list = SysDict::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("数据字典");

        // set column width
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 20.0 });

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row![
                "字典名称",
                "字典描述",
                "字典标签",
                "字典值",
                "创建日期"
            ])?;
            export_list.into_iter().for_each(|el| {
                let name = el.name.unwrap_or_default();
                let description = el.description.unwrap_or_default();
                let label = el.label.unwrap_or_default();
                let value = el.value.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![name, description, label, value, create_time]);
            });
            Ok(())
        })
            .expect("write excel error!");
        let data = wb.close().expect("close excel error!");

        if data.is_none() {
            return HttpResponse::InternalServerError().finish();
        }
        HttpResponse::Ok()
            .header("Content-Type", "application/octet-stream")
            .header(
                "Content-Disposition",
                format!(
                    "attachment;filename={}字典数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
