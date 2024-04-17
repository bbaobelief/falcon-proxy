use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{OnceLock, RwLock, RwLockReadGuard};

pub static APP_CONFIG: OnceLock<RwLock<AppConfig>> = OnceLock::new();

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppConfig {
    #[serde(default = "default_rust_log")]
    pub rust_log: String,
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    #[serde(default = "default_n9e_server")]
    pub n9e_server: String,
    pub falcon_agent_addr: Option<String>,
    pub monitor_company_abbr: Option<String>,
    #[serde(default = "default_allow_headers")]
    pub allow_headers: Vec<String>,
}

fn default_rust_log() -> String {
    "falcon=debug".to_string()
}

fn default_listen_addr() -> String {
    "127.0.0.1:1988".to_string()
}

fn default_n9e_server() -> String {
    "http://server.n9e.com".to_string()
}

fn default_allow_headers() -> Vec<String> {
    vec!["Monitor-Company-Abbr".to_string()]
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
