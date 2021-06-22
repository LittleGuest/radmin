use actix_web::{Responder, web};

use crate::commons::RespBuilder;
use crate::dtos::user::OnlineUserQuery;
use crate::REDIS_CLI;

pub struct OnlineController;

impl OnlineController {
    pub async fn online(_query: web::Query<OnlineUserQuery>) -> impl Responder {
        let mut conn = REDIS_CLI.get_connection().unwrap();

        let s: Vec<String> = redis::cmd("scan")
            .cursor_arg(0)
            .arg("match")
            .arg("online-token-*")
            .query(&mut conn)
            .unwrap();
        RespBuilder::ok().with_data(s).build()
    }

    pub async fn offline() -> impl Responder {
        RespBuilder::<&str>::ok().build()
    }
}
