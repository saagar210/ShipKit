use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "bad")]
pub enum BadEnum {
    A,
    B,
}

fn main() {}
