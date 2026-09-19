use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewGameRules {
    pub data: Value,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}
