use reqwest::header::HeaderMap;

pub struct ApiConfig {
    pub secret_key: String,
    pub public_key: String,
}

impl ApiConfig {
    pub fn create_header(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let authorization = format!("Bearer {}", self.secret_key);
        headers.insert("x-api-key", self.public_key.parse().unwrap());
        headers.insert("Authorization", authorization.parse().unwrap());
        return headers;
    }
}
