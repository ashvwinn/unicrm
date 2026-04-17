use std::sync::OnceLock;

struct AppConfig {
    api_url: String,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn init_config() {
    let api_url = std::env::var("API_URL").expect("API_URL environment variable not set");

    assert!(
        APP_CONFIG.set(AppConfig { api_url }).is_ok(),
        "Application config has already been initialized"
    );
}

pub fn api_url() -> &'static str {
    &APP_CONFIG
        .get()
        .expect("Application config is not initialized")
        .api_url
}
