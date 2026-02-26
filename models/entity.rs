use uuid::Uuid;
pub struct ShortUrl {
    pub id: Uuid,
    pub original_url: String,
    pub short_code: String,
}