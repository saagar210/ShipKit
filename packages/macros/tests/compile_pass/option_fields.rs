use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "options")]
pub struct OptionSettings {
    pub maybe_name: Option<String>,
    pub maybe_count: Option<i32>,
}

fn main() {}
