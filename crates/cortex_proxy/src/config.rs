use std::env;

pub struct Config {
    pub bind_addr: String,
    pub target_addr: String,
}

impl Config {
    pub fn new() -> Self {
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let target_port = env::var("TARGET_PORT").unwrap_or_else(|_| "3001".to_string());
        
        Self {
            bind_addr: format!("0.0.0.0:{}", port),
            target_addr: format!("127.0.0.1:{}", target_port),
        }
    }
}
