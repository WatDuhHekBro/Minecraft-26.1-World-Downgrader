use fastnbt::IntArray;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug)]
pub struct LevelDatDataPacks<'a> {
    #[serde(rename = "Disabled")]
    #[serde(borrow)]
    pub disabled: Vec<Cow<'a, str>>,

    #[serde(rename = "Enabled")]
    #[serde(borrow)]
    pub enabled: Vec<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LevelDatSpawn {
    pub dimension: String,
    pub pitch: f32,
    pub yaw: f32,
    pub pos: IntArray,
}

#[derive(Deserialize, Serialize, Debug)]
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
