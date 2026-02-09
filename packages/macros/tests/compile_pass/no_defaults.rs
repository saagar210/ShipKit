use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "nodefaults")]
pub struct NoDefaultSettings {
    pub name: String,
    #[settings(default = 10)]
    pub count: i32,
    pub flag: bool,
}

fn main() {}
