#[derive(Deserialize, Serialize, Debug)]
pub struct ToolLocalStorageQuery {
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
    // 文件名
    pub name: Option<String>,
}
