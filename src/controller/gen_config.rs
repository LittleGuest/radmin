use actix_web::{Responder, web};

use crate::commons::RespBuilder;
use crate::models::CodeGenConfig;

pub struct GenConfigController;

impl GenConfigController {
    pub async fn get_by_table_name(table_name: web::Path<String>) -> impl Responder {
        let data = CodeGenConfig::get_by_table_name(&table_name.0)
            .await
            .unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn update(form: web::Json<CodeGenConfig>) -> impl Responder {
        println!("form == {:?}", form);
        CodeGenConfig::update(form.0).await;
        RespBuilder::<&str>::ok().build()
    }
}
