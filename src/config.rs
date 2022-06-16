use reqwest::header::HeaderMap;

pub struct ApiConfig {
    pub secret_key: String,
    pub public_key: String,
    pub headers: HeaderMap,
    pub base_url: String
}

impl ApiConfig {

    pub fn new(secret_key: String, public_key: String, base_url: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let authorization = format!("Bearer {}", secret_key);
        headers.insert("x-api-key", public_key.parse().unwrap());
        headers.insert("Authorization", authorization.parse().unwrap());
        ApiConfig { secret_key, public_key, base_url, headers }
    }
}