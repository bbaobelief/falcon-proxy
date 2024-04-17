use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{OnceLock, RwLock, RwLockReadGuard};

pub static APP_CONFIG: OnceLock<RwLock<AppConfig>> = OnceLock::new();

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppConfig {
    pub rust_log: String,
    pub listen_addr: String,
    pub n9e_server: String,
    pub falcon_agent_addr: Option<String>,
    pub monitor_company_abbr: Option<String>,
    pub allow_headers: Vec<String>,
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl AppConfig {
    pub fn global() -> RwLockReadGuard<'static, AppConfig> {
        let lock = APP_CONFIG.get_or_init(|| {
            tracing::info!("read configuration.");
            let config_result = Config::builder()
                .add_source(File::with_name("etc/config").required(false))
                .add_source(Environment::default())
                .build()
                .expect("failed to build configuration");

            let app_config: AppConfig = config_result
                .try_deserialize()
                .expect("failed to deserialize AppConfig");
            RwLock::new(app_config)
        });
        lock.read()
            .expect("failed to acquire a read lock on AppConfig")
    }
}
