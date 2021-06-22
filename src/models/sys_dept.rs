use std::collections::HashSet;

use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::dept::{DeptExportDto, DeptForm, DeptQuery, DeptTree};
use crate::models::RBatisModel;
use crate::RB;

/// 部门
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SysDept {
    /// ID
    pub id: Option<i64>,
    /// 上级部门
    pub pid: Option<i64>,
    /// 子部门数目
    pub sub_count: Option<i32>,
    /// 名称
    pub name: Option<String>,
    /// 排序
    pub dept_sort: Option<i32>,
    /// 状态
    pub enabled: Option<u8>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysDept {
    pub async fn page(query: DeptQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.name.is_some() {
            wr.eq("name", query.name);
        }
        if query.enabled.is_some() {
            wr.eq("enabled", query.enabled);
        }
        if query.pid.is_some() {
            wr.eq("pid", query.pid);
        }
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

    pub async fn superior(ids: Vec<i64>) -> Result<Vec<DeptTree>> {
        let mut wr = RB.new_wrapper();
        wr.r#in("id", &ids);
        wr.check()?;
        let depts: Vec<Self> = RB.list_by_wrapper("", &wr).await?;

        let all = depts.iter().fold(HashSet::new(), |mut all, dept| {
            all.insert(dept.clone());
            if dept.pid.is_some() {
                let mut parents = HashSet::new();
                async {
                    Self::get_parents(dept.pid.unwrap_or(0), &mut parents).await;
                };
                parents.iter().for_each(|p| {
                    all.insert(p.clone());
                });
            }
            all
        });

        let temp = all.into_iter().fold(Vec::new(), |mut temp, item| {
            temp.push(DeptTree {
                id: item.id,
                pid: item.pid,
                sub_count: item.sub_count,
                name: item.name,
                dept_sort: item.dept_sort,
                enabled: item.enabled,
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

    async fn get_parents(pid: i64, p: &mut HashSet<Self>) {
        let mut wr = RB.new_wrapper();
        wr.eq("id", pid);
        let dept: Self = RB.fetch_by_wrapper("", &wr).await.unwrap_or_default();
        p.insert(dept.clone());
        if dept.pid.is_none() {
            return;
        }
        Self::get_parents(dept.pid.unwrap_or(0), p);
    }

    #[allow(unused_must_use)]
    pub async fn lit_children_id_by_pid(pid: i64, p: &mut HashSet<i64>) {
        let mut wr = RB.new_wrapper();
        wr.eq("pid", pid);
        let dept: Self = RB.fetch_by_wrapper("", &wr).await.unwrap_or_default();
        p.insert(dept.id.unwrap_or_default());
        if dept.pid.is_none() {
            return;
        }
        Self::lit_children_id_by_pid(dept.pid.unwrap_or(0), p);
    }

    // revert to tree data
    fn tree(all: Vec<DeptTree>) -> Vec<DeptTree> {
        let mut root = all
            .iter()
            .filter(|p| p.pid.is_none() || p.pid == Some(0))
            .cloned()
            .collect::<Vec<_>>();

        root.iter_mut().for_each(|p| {
            let children = all
                .iter()
                .filter(|c| p.id == c.pid)
                .cloned()
                .collect::<Vec<_>>();
            if !children.is_empty() {
                p.children = Some(children);
                p.has_child = Some(true);
            }
        });

        root
    }

    pub async fn export_list() -> Result<Vec<DeptExportDto>> {
        let sql = r#"
        SELECT
            sd.`name`,
            CASE
                WHEN sd.enabled = 1 THEN '启用'
                ELSE '停用'
            END AS enabled,
            sd.create_time
        FROM
            sys_dept sd
        ORDER BY
            sd.dept_sort
        "#;
        let export_list: Vec<DeptExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
