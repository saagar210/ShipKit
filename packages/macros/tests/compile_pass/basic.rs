use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "basic")]
pub struct BasicSettings {
    #[settings(default = "hello")]
    pub greeting: String,
    #[settings(default = true)]
    pub enabled: bool,
    #[settings(default = 42)]
    pub count: i64,
    #[settings(default = 1.5)]
    pub scale: f64,
}

fn main() {}
