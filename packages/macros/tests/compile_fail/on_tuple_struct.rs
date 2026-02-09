use serde::{Deserialize, Serialize};
use shipkit_core::Settings;

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "bad")]
pub struct TupleStruct(String, i32);

fn main() {}
