use actix_web::{Responder, web};

use crate::commons::RespBuilder;
use crate::controller::Controller;
use crate::dtos::dict::SysDictDetailQuery;
use crate::models::SysDictDetail;

pub struct DictDetailController;

impl Controller for DictDetailController {
    type M = SysDictDetail;
}

impl DictDetailController {
    pub async fn page(query: web::Query<SysDictDetailQuery>) -> impl Responder {
        let data = SysDictDetail::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn maps(query: web::Query<SysDictDetailQuery>) -> impl Responder {
        let dict_name = query.0.dict_name;
        if dict_name.is_none() {
            return RespBuilder::fail().build();
        }

        let dict_name = dict_name.unwrap();
        let dict_names: Vec<&str> = dict_name.split(',').collect();
        let data = SysDictDetail::maps(dict_names).await.unwrap_or_default();

        RespBuilder::ok().with_data(data).build()
    }
}
