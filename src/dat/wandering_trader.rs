use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWanderingTrader {
    data: NewWanderingTraderData,

    #[serde(rename = "DataVersion")]
    data_version: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewWanderingTraderData {
    spawn_delay: i32,
    spawn_chance: i32,
}
