pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5433/app".to_string())
}

pub fn get_api_prefix() -> String {
    std::env::var("API_PREFIX").unwrap_or_else(|_| "sk_live".to_string())
}
