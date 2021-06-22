#[derive(Serialize, Deserialize, Debug)]
pub struct CodeColumnConfigForm {
    // ID
    pub column_id: Option<i64>,
    // 表名
    pub table_name: Option<String>,
    // 数据库字段名称
    pub column_name: Option<String>,
    // 数据库字段类型
    pub column_type: Option<String>,
    // 字典名称
    pub dict_name: Option<String>,
    // 字段额外的参数
    pub extra: Option<String>,
    // 是否表单显示
    pub form_show: Option<u8>,
    // 表单类型
    pub form_type: Option<String>,
    // 数据库字段键类型
    pub key_type: Option<String>,
    // 是否在列表显示
    pub list_show: Option<u8>,
    // 是否必填
    pub not_null: Option<u8>,
    // 查询 1:模糊 2：精确
    pub query_type: Option<String>,
    // 数据库字段描述
    pub remark: Option<String>,
    // 日期注解
    pub date_annotation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeColumnConfigQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 表名
    pub table_name: Option<String>,
}
