use actix_http::{Error, http::StatusCode, Response};
use actix_web::{HttpRequest, Responder};
use futures_util::future::{ok, Ready};
use serde::Serialize;

#[derive(Serialize, Copy, Clone, Debug)]
pub struct Resp<T> {
    pub code: &'static str,
    pub msg: &'static str,
    pub data: Option<T>,
}

impl<T> Responder for Resp<T>
    where
        T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(Response::build(StatusCode::OK)
            .content_type("application/json")
            .json(self))
    }
}

#[derive(Serialize, Copy, Clone, Debug)]
pub struct RespBuilder<T>(pub Resp<T>);

impl<T> RespBuilder<T>
    where
        T: Serialize,
{
    pub fn new() -> Self {
        Self {
            0: Resp {
                code: "",
                msg: "",
                data: None,
            },
        }
    }

    pub fn ok() -> Self {
        Self {
            0: Resp {
                code: "200",
                msg: "请求成功",
                data: None,
            },
        }
    }

    pub fn fail() -> Self {
        Self {
            0: Resp {
                code: "500",
                msg: "服务器异常",
                data: None,
            },
        }
    }

    pub fn with_code(mut self, code: &'static str) -> Self {
        self.0.code = code;
        self
    }

    pub fn with_msg(mut self, msg: &'static str) -> Self {
        self.0.msg = msg;
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.0.data = Some(data);
        self
    }

    pub fn build(self) -> Resp<T> {
        self.0
    }
}
