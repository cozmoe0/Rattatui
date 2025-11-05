use common::api;
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

const EXTENSION_KEY_CODE: &str = "code";
const CODE_NOT_FOUND: &str = "NOT_FOUND";
const CODE_INTERNAL: &str = "INTERNAL";

impl From<crate::Error> for api::Error {
    fn from(err: crate::Error) -> Self {
        match err {
            crate::Error::NotFound(msg) => {
                let mut extensions = HashMap::with_capacity(1);
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_NOT_FOUND.into());
                api::Error {
                    message: msg,
                    extensions: Some(extensions),
                }
            }
            crate::Error::Internal(msg) => {
                let mut extensions = HashMap::with_capacity(1);
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_INTERNAL.into());
                api::Error {
                    message: msg,
                    extensions: Some(extensions),
                }
            }
            crate::Error::InvalidArgument(msg) => api::Error {
                message: msg,
                extensions: None,
            },
        }
    }
}

pub async fn handle_error(rejection: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (status, err) = if rejection.is_not_found() {
        (
            StatusCode::NOT_FOUND,
            crate::Error::NotFound("Route not found.".into()),
        )
    } else if rejection.find::<warp::filters::body::BodyDeserializeError>().is_some() {
        (
            StatusCode::BAD_REQUEST,
            crate::Error::InvalidArgument("Invalid Body.".into()),
        )
    } else if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            crate::Error::InvalidArgument("Invalid HTTP Method.".into()),
        )
    } else if let Some(e) = rejection.find::<crate::Error>() {
        let status = match e {
            crate::Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            crate::Error::NotFound(_) => StatusCode::NOT_FOUND,
            crate::Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, e.clone())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            crate::Error::Internal(String::new()),
        )
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::<()>::err(err.into())),
        status,
    ))
}
