use std::env;

#[derive(Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(3000);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self { host, port, database_url }
    }
}