use chrono::NaiveDateTime;

use crate::models::{SysDept, SysMenu};

#[derive(Serialize, Deserialize)]
pub struct RoleQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 名称或描述
    pub blurry: Option<String>,
    // 开始时间
    pub start_date: Option<String>,
    // 结束时间
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoleForm {
    /// ID
    pub id: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 角色级别
    pub level: Option<i32>,
    /// 描述
    pub description: Option<String>,
    /// 数据权限
    pub data_scope: Option<String>,

    // 数据权限
    pub depts: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoleMenuForm {
    // 角色ID
    pub role_id: Option<i64>,
    // 菜单IDS
    pub menus: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoleDto {
    // ID
    pub role_id: Option<i64>,
    // 名称
    pub name: Option<String>,
    // 角色级别
    pub level: Option<i64>,
    // 描述
    pub description: Option<String>,
    // 数据权限
    pub data_scope: Option<String>,

    // 创建人
    pub create_by: Option<String>,
    // 更新人
    pub update_by: Option<String>,
    // 创建时间
    pub create_time: Option<NaiveDateTime>,
    // 更新时间
    pub update_time: Option<NaiveDateTime>,

    pub depts: Option<SysDept>,
    pub menus: Option<SysMenu>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleExportDto {
    // 名称
    pub name: Option<String>,
    // 角色级别
    pub level: Option<i64>,
    // 描述
    pub description: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserRoleDto {
    // ID
    pub role_id: Option<i64>,
    // 名称
    pub name: Option<String>,
    // 角色级别
    pub level: Option<i64>,
    // 数据权限
    pub data_scope: Option<String>,
}
