use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct DeptQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 上级部门
    pub pid: Option<i64>,
    // 名称
    pub name: Option<i64>,
    // 排序
    pub dept_sort: Option<i64>,
    // 状态
    pub enabled: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct DeptForm {
    /// ID
    pub id: Option<i64>,
    /// 上级部门
    pub pid: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 排序
    pub dept_sort: Option<i32>,
    /// 状态
    pub enabled: Option<u8>,

    // 是否是顶级部门
    pub is_top: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeptExportDto {
    // 名称
    pub name: Option<String>,
    // 状态
    pub enabled: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDeptDto {
    // ID
    pub dept_id: Option<i64>,
    // 名称
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DeptTree {
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

    /// 是否有子
    pub has_child: Option<bool>,
    /// 子
    pub children: Option<Vec<DeptTree>>,
}
