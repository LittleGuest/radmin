#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeGenConfigForm {
    // ID
    pub config_id: Option<i64>,
    // 表名
    pub table_name: Option<String>,
    // 作者
    pub author: Option<String>,
    // 是否覆盖
    pub cover: Option<u8>,
    // 模块名称
    pub module_name: Option<String>,
    // 至于哪个包下
    pub pack: Option<String>,
    // 前端代码生成的路径
    pub path: Option<String>,
    // 前端Api文件路径
    pub api_path: Option<String>,
    // 表前缀
    pub prefix: Option<String>,
    // 接口名称
    pub api_alias: Option<String>,
}
