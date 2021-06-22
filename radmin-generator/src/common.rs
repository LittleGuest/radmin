// 暂时只有 mysql
pub(crate) const DRIVER: &str = "mysql";
pub(crate) const HOST: &str = "localhost";
pub(crate) const PORT: &str = "3306";
pub(crate) const USERNAME: &str = "root";
pub(crate) const PASSWORD: &str = "root";
pub(crate) const DATABASE: &str = "radmin";

pub(crate) const MODEL_TEMPLATE: &str = r#"
use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// {{table.table_comment}}
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct {{ struct_name }} { {% if has_columns %}{% for column in columns %}
    /// {{column.column_comment}}
    pub {{column.column_name}}: Option<{{column.field_type}}>,{% endfor %}{% endif %}
}
"#;

pub(crate) const MOD_TEMPLATE: &str = r#"
{% for table_name, _ in table_names %}
mod {{table_name}};
pub use {{table_name}}::*;
{% endfor %}
"#;
