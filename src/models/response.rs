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
    pub error: Option<ApiErrorResponse>,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
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
            error: Some(ApiErrorResponse { code, message }),
        }
    }
}
