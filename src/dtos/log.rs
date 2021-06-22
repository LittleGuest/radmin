#[derive(Serialize, Deserialize, Default)]
pub struct LogQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 日志类型
    pub log_type: Option<String>,
    // 用户ID
    pub username: Option<i64>,

    // 名称或描述
    pub blurry: Option<String>,
    // 开始时间
    pub start_date: Option<String>,
    // 结束时间
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LogExportDto {
    // 用户名
    pub username: Option<String>,
    // 请求IP地址
    pub request_ip: Option<String>,
    // 地址
    pub address: Option<String>,
    // 描述
    pub description: Option<String>,
    // 浏览器
    pub browser: Option<String>,
    // 用时
    pub time: Option<i64>,
    // 异常信息
    pub exception_detail: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}
