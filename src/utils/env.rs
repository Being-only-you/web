use std::env;

pub fn get_env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).expect(&format!("ENVIRONMENT ERROR: {} not set!", key))
}

pub fn get_env_or_none(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}