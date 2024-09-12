use crate::utils::env::{get_env, get_env_or, get_env_or_none};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_name: String,
    pub database_namespace: String,
    pub database_username: String,
    pub database_password: String,
    pub database_endpoint: String,
    pub auth_cookie_key: String,
    pub cookie_domain: Option<String>,
    pub domain: String
}

impl AppConfig {
    /// Initializes dotenv and returns a `AppConfig` containing all the required variables
    pub fn init() -> Self {
        let database_namespace = get_env("SURREAL_NAMESPACE");
        let database_name = get_env("SURREAL_DATABASE");
        let database_username = get_env("SURREAL_USERNAME");
        let database_password = get_env("SURREAL_PASSWORD");
        let database_endpoint = get_env_or("SURREAL_ENDPOINT", "127.0.0.1:8000");
        let auth_cookie_key = get_env_or("AUTH_COOKIE_NAME", "");
        AppConfig {
            database_name,
            database_namespace,
            database_password,
            database_username,
            database_endpoint,
            auth_cookie_key,
            domain: get_env_or("DOMAIN", "localhost:3000"),
            cookie_domain: get_env_or_none("COOKIE_DOMAIN")
        }
    }
}