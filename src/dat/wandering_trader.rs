use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWanderingTrader {
    pub data: NewWanderingTraderData,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWanderingTraderData {
    pub spawn_delay: i32,
    pub spawn_chance: i32,
}
