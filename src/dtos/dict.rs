#[derive(Deserialize, Serialize, Debug)]
pub struct SysDictQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    // pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 名称或描述
    pub blurry: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SysDictDetailQuery {
    // 当前页数
    pub current: Option<u64>,
    // 每页显示数量
    pub size: Option<u64>,
    // 排序字段
    // pub sort: Option<String>,
    // 排序方式
    pub is_asc: Option<bool>,

    // 字典id
    pub dict_id: Option<i64>,
    // 字典名称
    pub dict_name: Option<String>,
    // 字典标签
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DictDetailDto {
    // ID
    pub id: Option<i64>,
    // 字典id
    pub dict_id: Option<i64>,
    // 字典标签
    pub label: Option<String>,
    // 字典值
    pub value: Option<String>,
    // 排序
    pub dict_sort: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct DictDetailExportDto {
    // 字典名称
    pub name: Option<String>,
    // 字典描述
    pub description: Option<String>,
    // 字典标签
    pub label: Option<String>,
    // 字典值
    pub value: Option<String>,
    // 创建时间
    pub create_time: Option<String>,
}
