use thiserror::Error;

use anyhow::Error;
use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod log;

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Res<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(error: Error) -> Self {
        let code = if error.downcast_ref::<CusErr>().is_some() {
            match error.downcast_ref::<CusErr>() {
                Some(e) => match e {
                    CusErr::InternalServerErr(_) => 500,
                    CusErr::NotFoundErr(_) => 404,
                    CusErr::BadRequestErr(_) => 400,
                    CusErr::UnauthorizedErr(_) => 401,
                    CusErr::ForbiddenErr(_) => 403,
                    CusErr::ReqParamErr(_) => 400,
                },
                _ => 500,
            }
        } else {
            500
        };

        Self {
            code,
            message: error.to_string(),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> Response {
        let val = json!(self);

        Json(val).into_response()
    }
}

impl From<JsonRejection> for Res<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            message: value.body_text(),
            data: None,
        }
    }
}

impl From<QueryRejection> for Res<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            message: value.body_text(),
            data: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum CusErr {
    #[error("req param error: {0}")]
    ReqParamErr(String),
    #[error("internal server error: {0}")]
    InternalServerErr(String),
    #[error("not found: {0}")]
    NotFoundErr(String),
    #[error("bad request: {0}")]
    BadRequestErr(String),
    #[error("unauthorized: {0}")]
    UnauthorizedErr(String),
    #[error("forbidden: {0}")]
    ForbiddenErr(String),
}
