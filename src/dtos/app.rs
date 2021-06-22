#[derive(Serialize, Deserialize)]
pub struct AppQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 名称
    pub name: Option<i64>,
    // 开始时间
    pub start_date: Option<String>,
    // 结束时间
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppExportDto {
    // 应用名称
    pub name: Option<String>,
    // 应用端口
    pub port: Option<i16>,
    // 上传目录
    pub upload_path: Option<String>,
    // 部署路径
    pub deploy_path: Option<String>,
    // 备份路径
    pub backup_path: Option<String>,
    // 启动脚本
    pub start_script: Option<String>,
    // 部署脚本
    pub deploy_script: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}
