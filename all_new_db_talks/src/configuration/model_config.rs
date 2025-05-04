use std::env;

pub enum ModelSelect {
    SqlOperate,
    NplOperate,
    TinyLlma
}

impl ModelSelect {
    pub fn as_str(&self) -> String {
        let sql = env::var("SQL_OPERATE").unwrap_or_else(|_| "sqlcoder:7b".to_string());
        let npl = env::var("NPL_OPERATE").unwrap_or_else(|_| "llama3.1:8b".to_string());
        let tiny_llama = env::var("TINY_LLAMA").unwrap_or_else(|_| "timyllama:latest".to_string());
        match self {
            ModelSelect::SqlOperate => sql,
            ModelSelect::NplOperate => npl,
            ModelSelect::TinyLlma => tiny_llama
        }
    }
}