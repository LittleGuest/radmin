use std::collections::HashSet;

use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::menu::{MenuExportDto, MenuQuery, MenuTree};
use crate::models::RBatisModel;
use crate::RB;

/// 系统菜单
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SysMenu {
    /// ID
    pub id: Option<i64>,
    /// 上级菜单ID
    pub pid: Option<i64>,
    /// 子菜单数目
    pub sub_count: Option<i32>,
    /// 菜单类型
    pub r#type: Option<i32>,
    /// 菜单标题
    pub title: Option<String>,
    /// 组件名称
    pub name: Option<String>,
    /// 组件
    pub component: Option<String>,
    /// 排序
    pub menu_sort: Option<i32>,
    /// 图标
    pub icon: Option<String>,
    /// 链接地址
    pub path: Option<String>,
    /// 是否外链
    pub i_frame: Option<u8>,
    /// 缓存
    pub cache: Option<u8>,
    /// 隐藏
    pub hidden: Option<u8>,
    /// 权限
    pub permission: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysMenu {
    pub async fn page(query: MenuQuery) -> Result<Page<MenuTree>> {
        let sql = r#"
        SELECT
            id,
            pid,
            sub_count,
            type,
            title,
            `name`,
            component,
            menu_sort,
            icon,
            path,
            i_frame,
            `cache`,
            hidden,
            permission,
            create_by,
            update_by,
            create_time,
            update_time 
        FROM
            sys_menu
        WHERE
            1 = 1 
            if title != null:
                AND INSTR(title, #{title})
            if pid != null:
                AND pid = #{pid}
            choose:
                when pid != null:
                    AND pid = #{pid}
                otherwise:
                    AND pid = 0
        ORDER BY
            menu_sort        
        "#;
        let mut pages: Page<MenuTree> = RB
            .py_fetch_page(
                "",
                sql,
                &json!({
                    "title" : query.blurry,
                    "pid" : query.pid,
                }),
                &PageRequest::new(query.current.unwrap_or(1), query.size.unwrap_or(20)),
            )
            .await?;

        if !pages.records.is_empty() {
            pages.records.iter_mut().for_each(|r| {
                r.has_children = Some(!(r.sub_count.is_none() || Some(0) == r.sub_count));
            });
        }
        Ok(pages)
    }

    pub async fn list(ids: Option<&[i64]>) -> Result<Vec<Self>> {
        let mut wr = RB.new_wrapper();
        if let Some(ids) = ids {
            wr.r#in("id", ids);
        }
        wr.order_by(true, &["menu_sort"]);
        wr.check()?;
        let menus: Vec<Self> = RB.list_by_wrapper("", &wr).await?;
        Ok(menus)
    }

    pub async fn export_list() -> Result<Vec<MenuExportDto>> {
        let sql = r#"
        SELECT
            sm.title ,
            case
                when sm.`type` = 0 then '目录'
                when sm.`type` = 1 then '菜单'
                when sm.`type` = 2 then '按钮'
                else ''
            end as type,
            sm.permission ,
            if(sm.i_frame = 1,
            '否',
            '是') as i_frame,
            if(sm.hidden = 1,
            '否',
            '是') as hidden,
            if(sm.cache = 1,
            '否',
            '是') as cache,
            sm.create_time
        FROM
            sys_menu sm
        order by
            sm.menu_sort
        "#;
        let export_list: Vec<MenuExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }

    pub async fn child(pid: Option<i64>) -> Result<Vec<i64>> {
        let mut wr = RB.new_wrapper();
        if pid.is_none() || pid == Some(0) {
            wr.is_null("pid");
        } else {
            wr.eq("pid", pid.unwrap_or_default());
        }
        wr.check()?;
        let children: Vec<Self> = RB.list_by_wrapper("", &wr).await?;

        let children_ids: Vec<i64> = children.iter().map(|c| c.id.unwrap_or_default()).collect();
        Ok(children_ids)
    }

    pub async fn superior(ids: &[i64]) -> Result<Vec<MenuTree>> {
        let menus = Self::list(Option::from(ids)).await?;

        let all = menus.iter().fold(HashSet::new(), |mut all, menu| {
            all.insert(menu.clone());

            if menu.pid.is_none() || menu.pid == Some(0) {
                return all;
            }
            let mut parents = HashSet::new();
            async {
                Self::get_parents(menu.pid.unwrap_or(0), &mut parents).await;
            };
            parents.iter().for_each(|p| {
                all.insert(p.clone());
            });
            all
        });

        let temp = all.into_iter().fold(Vec::new(), |mut temp, item| {
            temp.push(MenuTree {
                id: item.id,
                pid: item.pid,
                sub_count: item.sub_count,
                r#type: item.r#type,
                title: item.title,
                name: item.name,
                component: item.component,
                menu_sort: item.menu_sort,
                icon: item.icon,
                path: item.path,
                i_frame: item.i_frame,
                cache: item.cache,
                hidden: item.hidden,
                permission: item.permission,
                create_by: item.create_by,
                update_by: item.update_by,
                create_time: item.create_time,
                update_time: item.update_time,
                ..Default::default()
            });
            temp
        });

        Ok(Self::tree(temp))
    }

    #[allow(unused_must_use)]
    async fn get_parents(pid: i64, p: &mut HashSet<Self>) {
        let mut wr = RB.new_wrapper();
        wr.eq("id", pid);
        let menu: Self = RB.fetch_by_wrapper("", &wr).await.unwrap_or_default();
        p.insert(menu.clone());
        if menu.pid.is_none() {
            return;
        }
        Self::get_parents(menu.pid.unwrap_or(0), p);
    }

    // revert to tree data
    fn tree(all: Vec<MenuTree>) -> Vec<MenuTree> {
        let mut root = all
            .iter()
            .filter(|p| p.pid.is_none() || p.pid == Some(0))
            .cloned()
            .collect::<Vec<_>>();

        root.iter_mut().for_each(|p| {
            let mut children = all
                .iter()
                .filter(|c| p.id == c.pid)
                .cloned()
                .collect::<Vec<_>>();
            if !children.is_empty() {
                p.children = Some(children);
                p.has_children = Some(true);
            }
        });

        root
    }

    pub async fn lazy(query: MenuQuery) -> Result<Vec<Self>> {
        let mut wr = RB.new_wrapper();
        if query.pid.is_none() || query.pid == Some(0) {
            wr.is_null("pid");
        } else {
            wr.eq("pid", query.pid.unwrap_or_default());
        }
        wr.order_by(true, &["menu_sort"]);
        wr.check()?;
        let menus: Vec<Self> = RB.list_by_wrapper("", &wr).await?;
        Ok(menus)
    }
}
