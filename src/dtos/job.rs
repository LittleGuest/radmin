#[derive(Serialize, Deserialize)]
pub struct JobQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    // pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 岗位名称
    pub name: Option<String>,
    // 岗位状态
    pub enabled: Option<u8>,
    // 排序
    pub job_sort: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct JobExportDto {
    // 岗位名称
    pub name: Option<String>,
    // 岗位状态
    pub enabled: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserJobDto {
    // ID
    pub job_id: Option<i64>,
    // 岗位名称
    pub name: Option<String>,
}
