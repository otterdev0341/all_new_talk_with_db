use std::env;

pub struct LLMConfig{
    pub url: String,
    pub port: u16
}

impl LLMConfig{
    pub fn inject_from_env() -> Self {
        dotenv::dotenv().ok();
        let ollama_url = match env::var("OLAMA_URL") {
            Ok(data) => data,
            Err(_) => "".to_string()
        };
        let ollama_port: u16 = match env::var("OLAMA_PORT") {
            Ok(data) => data.parse().unwrap(),
            Err(_) => 0
        };
        Self { url: ollama_url, port: ollama_port }
    }
}