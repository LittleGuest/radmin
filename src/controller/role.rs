use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, web};
use anyhow::{Error, Result};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::dtos::role::{RoleForm, RoleMenuForm, RoleQuery};
use crate::models::{SysRole, SysRolesMenus};

pub struct RoleController;

impl RoleController {
    pub async fn page(query: web::Query<RoleQuery>) -> impl Responder {
        let data = SysRole::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn save(form: web::Json<RoleForm>) -> impl Responder {
        SysRole::save(form.0).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn update(form: web::Json<RoleForm>) -> impl Responder {
        let form = form.0;
        if form.id.is_none() {
            return RespBuilder::ok().with_msg("ID为空").build();
        }
        SysRole::update(form).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn delete_batch(ids: web::Json<Vec<i64>>) -> impl Responder {
        let ids = ids.0;
        SysRole::delete_batch(ids).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn get_by_id(id: web::Path<i64>) -> impl Responder {
        let data = SysRole::get_by_id(id.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn all() -> impl Responder {
        let data = SysRole::all().await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn level() -> impl Responder {
        // TODO 获取用户ID
        let user_id = 0;
        match Self::get_levels(user_id, None).await {
            Ok(l) => {
                let mut level_map = HashMap::with_capacity(1);
                level_map.insert("level", l);
                RespBuilder::ok().with_data(level_map).build()
            }
            Err(_e) => RespBuilder::fail().with_msg("权限不足").build(),
        }
    }

    async fn get_levels(user_id: i64, level: Option<i64>) -> Result<Option<i64>> {
        let levels = SysRole::level(user_id).await.unwrap_or_default();
        let min: Option<i64> = levels.iter().min().and(None);

        if let Some(l) = level {
            if let Some(m) = min {
                if l < m {
                    return Err(Error::msg(format!(
                        "权限不足，你的角色级别：{}，低于操作的角色级别：{}",
                        m, l
                    )));
                }
            }
        }
        Ok(min)
    }

    pub async fn role_menu(form: web::Json<RoleMenuForm>) -> impl Responder {
        let form = form.0;
        if form.role_id.is_none() || form.menus.is_none() {
            return RespBuilder::<&str>::fail().with_msg("参数为空").build();
        }

        SysRolesMenus::delete_batch_by_role(form.role_id.unwrap_or_default()).await;

        let mut role_menus = Vec::new();
        for menu_id in form.menus.unwrap_or_default() {
            let role_menu = SysRolesMenus {
                role_id: form.role_id,
                menu_id: Option::from(menu_id),
            };
            role_menus.push(role_menu);
        }

        SysRolesMenus::save_batch(&role_menus).await;
        // TODO 刷新用户菜单缓存
        RespBuilder::<&str>::ok().build()
    }

    pub async fn export() -> impl Responder {
        let export_list = SysRole::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("角色");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["角色名称", "角色级别", "描述", "创建日期"])?;
            export_list.into_iter().for_each(|el| {
                let name = el.name.unwrap_or_default();
                let level = el.level.unwrap_or_default().to_string();
                let description = el.description.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![name, level, description, create_time]);
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
                    "attachment;filename={}角色数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
