use fastnbt::IntArray;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LevelDatDataPacks {
    #[serde(rename = "Disabled")]
    pub disabled: Vec<String>,

    #[serde(rename = "Enabled")]
    pub enabled: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LevelDatSpawn {
    pub dimension: String,
    pub pitch: f32,
    pub yaw: f32,
    pub pos: IntArray,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LevelDatVersionInfo {
    #[serde(rename = "Id")]
    pub id: i32,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Series")]
    pub series: String,

    #[serde(rename = "Snapshot")]
    pub snapshot: bool,
}
