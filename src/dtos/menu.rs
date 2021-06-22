use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct MenuQuery {
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
    // 上级菜单ID
    pub pid: Option<i64>,
    // 开始时间
    pub start_date: Option<String>,
    // 结束时间
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuExportDto {
    // 菜单标题
    pub title: Option<String>,
    // 菜单类型
    pub r#type: Option<String>,
    // 权限
    pub permission: Option<String>,
    // 是否外链
    pub i_frame: Option<String>,
    // 隐藏
    pub hidden: Option<String>,
    // 缓存
    pub cache: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MenuTree {
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

    /// 是否有子
    pub has_children: Option<bool>,
    /// 子
    pub children: Option<Vec<MenuTree>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserMenuDto {
    // 组件名称
    pub name: Option<String>,
    // 链接地址
    pub path: Option<String>,
    // 隐藏
    pub hidden: Option<bool>,
    // 是否外链
    pub redirect: Option<String>,
    // 组件
    pub component: Option<String>,
    pub always_show: Option<bool>,
    pub meta: Option<MenuMetaDto>,
    pub children: Option<Vec<UserMenuDto>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MenuMetaDto {
    // 菜单标题
    pub title: Option<String>,
    // 图标
    pub icon: Option<String>,
    // 缓存
    pub no_cache: Option<bool>,
}
