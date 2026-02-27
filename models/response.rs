use std::os::unix::net::Messages;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateShortUrlResponse {
    pub url: String,
    pub short_code: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<T>,
}

pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}

impl ApiResponse {
    pub fn success(data: Option<T>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    pub fn error(code: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError { code, message }),
        }
    }
}
