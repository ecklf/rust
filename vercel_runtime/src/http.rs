use lambda_http::{http::StatusCode, Body, Error, Response};
use serde::Serialize;
use std::error::Error as StdError;
use tracing::error;

pub fn ok<T: Serialize>(val: T) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::Text(serde_json::to_string(&val).unwrap()))?)
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

impl From<APIError> for lambda_http::Body {
    fn from(val: APIError) -> Self {
        lambda_http::Body::Text(serde_json::to_string(&val).unwrap())
    }
}

pub fn bad_request(message: &'static str) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("content-type", "application/json")
        .body(
            APIError {
                message,
                code: "bad_request",
            }
            .into(),
        )?)
}

#[derive(Serialize)]
struct Found {
    found: bool,
}

impl From<Found> for lambda_http::Body {
    fn from(val: Found) -> Self {
        lambda_http::Body::Text(serde_json::to_string(&val).unwrap())
    }
}

const NOT_FOUND: Found = Found { found: false };

pub fn not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(NOT_FOUND.into())?)
}

pub fn endpoint_not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(
            APIError {
                message: "Not found",
                code: "not_found",
            }
            .into(),
        )?)
}

pub fn unauthorized() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("content-type", "application/json")
        .body(
            APIError {
                message: "Unauthorized",
                code: "unauthorized",
            }
            .into(),
        )?)
}

pub fn internal_server_error(err: impl StdError) -> Result<Response<Body>, Error> {
    error!(error = ?err, "internal server error");
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("content-type", "application/json")
        .body(
            APIError {
                message: "Internal server error",
                code: "internal_server_error",
            }
            .into(),
        )?)
}
