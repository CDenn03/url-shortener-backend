use serde::Serialize;

#[derive(Serialize)]
pub struct CreateShortUrlResponse {
    pub url: String,
    pub short_code: String,
}

#[derive(Serialize)]
pub struct ApiResponse {

}

#[derive(Serialize)]
pub struct ApiErrorBody {

}
