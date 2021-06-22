#![allow(unused)]

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use thiserror::Error;

pub use base::*;
pub use common::*;

mod base;
mod common;

// TODO custom response error
#[derive(Debug, Error)]
pub enum RespErr {
    #[error("服务器异常")]
    InternalError,
    #[error("请求超时")]
    Timeout,
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl ResponseError for RespErr {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(RespBuilder::<&str>::fail().build())
    }
}
