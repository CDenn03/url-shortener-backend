use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateShortUrlRequest {
    pub url: String,
    pub short_code: Option<String>,
}