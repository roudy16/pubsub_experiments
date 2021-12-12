use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub log_level: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Logging,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            logging: Logging {
                log_level: String::from("debug"),
            },
        }
    }
}
