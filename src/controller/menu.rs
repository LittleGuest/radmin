use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::controller::Controller;
use crate::dtos::menu::{MenuMetaDto, MenuQuery, MenuTree, UserMenuDto};
use crate::models::{SysMenu, SysRolesMenus, SysUser};

pub struct MenuController;

impl Controller for MenuController {
    type M = SysMenu;
}

impl MenuController {
    pub async fn page(query: web::Query<MenuQuery>) -> impl Responder {
        let data = SysMenu::page(query.0).await.unwrap();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn build(session: Session) -> impl Responder {
        let user_id = session.get::<i64>("user_id").unwrap_or_default();
        if let Some(user_id) = user_id {
            //  获取用户角色
            let user_roles = SysUser::user_roles(user_id).await.unwrap_or_default();
            let role_ids: Vec<i64> = user_roles.iter().map(|urd| urd.role_id.unwrap()).collect();
            let menu_ids = SysRolesMenus::roles_menus(&role_ids)
                .await
                .unwrap_or_default();
            //  获取角色菜单
            let menu_tree = SysMenu::superior(&menu_ids).await.unwrap_or_default();

            // packaging data
            let data = Self::tree_to_tree(menu_tree);
            return RespBuilder::ok().with_data(data).build();
        }
        RespBuilder::ok().with_data(vec![]).build()
    }

    // revert to tree data
    fn tree_to_tree(mts: Vec<MenuTree>) -> Vec<UserMenuDto> {
        mts.into_iter().fold(Vec::new(), |mut umds, mt| {
            let children = mt.children.unwrap_or_default();

            let is_parent = mt.pid.is_none() || mt.pid == Some(0);
            let is_iframe = {
                let mut iframe = false;
                if let Some(ife) = mt.i_frame {
                    if ife == 1 {
                        iframe = true
                    }
                }
                iframe
            };
            let is_hidden = {
                let mut hidden = false;
                if let Some(h) = mt.hidden {
                    if h == 1 {
                        hidden = true;
                    }
                }
                hidden
            };

            let mut umd = UserMenuDto {
                name: {
                    if let Some(component) = &mt.component {
                        if component.is_empty() {
                            mt.title.clone()
                        } else {
                            Some(component.to_string())
                        }
                    } else {
                        mt.title.clone()
                    }
                },
                // 一级目录需要加斜杠，不然会报警告
                path: {
                    if is_parent {
                        Option::from(format!("/{}", mt.path.unwrap_or_default()))
                    } else {
                        mt.path
                    }
                },
                hidden: Some(is_hidden),
                component: {
                    if !is_iframe {
                        let mut component = "";
                        let is_component_empty = {
                            if mt.component.is_none() {
                                true
                            } else if let Some(cmpt) = &mt.component {
                                component = cmpt;
                                cmpt.is_empty()
                            } else {
                                false
                            }
                        };
                        if is_parent {
                            if is_component_empty {
                                Some("Layout".to_string())
                            } else {
                                Some(component.to_string())
                            }
                        } else if Some(0) == mt.r#type {
                            if is_component_empty {
                                Some("ParentView".to_string())
                            } else {
                                Some(component.to_string())
                            }
                        } else if mt.component.is_some() {
                            mt.component.clone()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                meta: Option::from(MenuMetaDto {
                    title: mt.title.clone(),
                    icon: mt.icon,
                    no_cache: {
                        let mut cache = false;
                        if let Some(c) = mt.cache {
                            if c == 1 {
                                cache = true;
                            }
                        }
                        Option::from(!cache)
                    },
                }),
                ..Default::default()
            };

            if !children.is_empty() {
                umd.always_show = Some(true);
                umd.redirect = Some("noredirect".to_string());
                umd.children = Option::from(Self::tree_to_tree(children));
            } else if is_parent {
                // 处理是一级菜单并且没有子菜单的情况

                let mut umd_p = UserMenuDto {
                    meta: umd.meta,
                    ..Default::default()
                };

                if !is_iframe {
                    umd_p.path = Some("index".to_string());
                    umd_p.name = umd.name;
                    umd_p.component = umd.component;
                } else {
                    umd_p.path = umd.path.clone()
                }
                umd.name = None;
                umd.meta = None;
                umd.component = Some("Layout".to_string());
                umd.children = Some(vec![umd_p]);
            }
            umds.push(umd.clone());
            umds
        })
    }

    pub async fn child(query: web::Query<MenuQuery>) -> impl Responder {
        let pid = query.0.pid;
        let mut data = SysMenu::child(pid).await.unwrap_or_default();
        if let Some(pid) = pid {
            data.push(pid);
        }
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn superior(ids: web::Json<Vec<i64>>) -> impl Responder {
        let ids = ids.0;
        let data = SysMenu::superior(&ids).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn lazy(query: web::Query<MenuQuery>) -> impl Responder {
        let data = SysMenu::lazy(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn export() -> impl Responder {
        let export_list = SysMenu::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("菜单");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row![
                "菜单标题",
                "菜单类型",
                "权限标识",
                "外链菜单",
                "菜单可见",
                "是否缓存",
                "创建日期"
            ])?;
            export_list.into_iter().for_each(|el| {
                let title = el.title.unwrap_or_default();
                let r#type = el.r#type.unwrap_or_default();
                let permission = el.permission.unwrap_or_default();
                let i_frame = el.i_frame.unwrap_or_default();
                let hidden = el.hidden.unwrap_or_default();
                let cache = el.cache.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![
                    title,
                    r#type,
                    permission,
                    i_frame,
                    hidden,
                    cache,
                    create_time
                ]);
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
                    "attachment;filename={}菜单数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
