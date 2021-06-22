use std::collections::HashSet;
use std::fmt::Display;

use serde::export::Formatter;
use validator::Validate;

use crate::dtos::dept::UserDeptDto;
use crate::dtos::job::UserJobDto;
use crate::dtos::role::UserRoleDto;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CurrentUser {
    // ID
    pub user_id: i64,
    // 部门
    pub dept_id: Option<i64>,
    // 用户名
    pub username: String,
    // 昵称
    pub nick_name: Option<String>,
    // 是否为admin账号
    pub is_admin: Option<u8>,

    // 浏览器
    pub browser: Option<String>,
    // IP
    pub ip: Option<String>,
    // 地址
    pub address: Option<String>,
    // token
    pub key: Option<String>,
    // 登录时间
    pub login_time: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(required)]
    // 用户名
    pub username: Option<String>,
    // 密码
    pub password: Option<String>,
    // 验证码
    pub code: Option<String>,
    // 验证码ID
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginDto {
    pub data_scopes: Option<HashSet<i64>>,
    pub roles: Option<HashSet<String>>,
    pub user: Option<UserDto>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    /// ID
    pub id: Option<i64>,
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nick_name: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 头像地址
    pub avatar_name: Option<String>,
    /// 头像真实路径
    pub avatar_path: Option<String>,
    /// 状态：1启用、0禁用
    pub enabled: Option<i64>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新着
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,

    pub dept: Option<UserDeptDto>,
    pub jobs: Vec<UserJobDto>,
    pub roles: Vec<UserRoleDto>,
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 部门
    pub dept_id: Option<i64>,
    // 名称或描述
    pub blurry: Option<String>,
    // 开始时间
    pub start_date: Option<String>,
    // 结束时间
    pub end_date: Option<String>,
    // 状态：1启用、0禁用
    pub enabled: Option<u8>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserForm {
    /// ID
    pub id: Option<i64>,
    /// 部门名称
    pub dept_id: Option<i64>,
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nick_name: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 状态：1启用、0禁用
    pub enabled: Option<i64>,

    // 岗位
    pub jobs: Option<Vec<i64>>,
    // 角色
    pub roles: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserExportDto {
    // 用户名
    pub username: Option<String>,
    // 角色
    pub role: Option<String>,
    // 部门
    pub dept: Option<String>,
    // 岗位
    pub job: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 状态
    pub enabled: Option<String>,
    // 手机号码
    pub phone: Option<String>,
    // 修改密码的时间
    pub pwd_reset_time: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEmailForm {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePassForm {
    // 旧密码
    pub old_pass: String,
    // 新密码
    pub new_pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCenterForm {
    // 昵称
    pub nick_name: Option<String>,
    // 性别
    pub gender: Option<String>,
    // 手机号码
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OnlineUserQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    pub filter: Option<String>,
}
