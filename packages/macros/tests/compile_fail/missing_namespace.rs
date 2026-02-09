use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
pub struct BadSettings {
    pub name: String,
}

fn main() {}
