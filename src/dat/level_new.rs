use fastnbt::IntArray;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewLevelDat<'a> {
    #[serde(rename = "Data")]
    #[serde(borrow)]
    data: NewLevelDatData<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewLevelDatData<'a> {
    #[serde(rename = "allowCommands")]
    allow_commands: bool,

    #[serde(rename = "DataPacks")]
    #[serde(borrow)]
    datapacks: Option<NewLevelDatDataPacks<'a>>,

    #[serde(rename = "DataVersion")]
    data_version: i32,

    #[serde(borrow)]
    difficulty_settings: Option<NewLevelDatDifficultySettings<'a>>,

    #[serde(borrow)]
    enabled_features: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "GameType")]
    game_type: i32,

    initialized: bool,

    #[serde(rename = "LastPlayed")]
    last_played: i64,

    #[serde(rename = "LevelName")]
    #[serde(borrow)]
    level_name: Cow<'a, str>,

    singleplayer_uuid: Option<IntArray>,

    #[serde(rename = "ServerBrands")]
    #[serde(borrow)]
    server_brands: Option<Vec<Cow<'a, str>>>,

    spawn: NewLevelDatSpawn,

    #[serde(rename = "Time")]
    time: i64,

    version: i32,

    #[serde(rename = "Version")]
    version_info: NewLevelDatVersionInfo,

    #[serde(rename = "WasModded")]
    was_modded: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewLevelDatDataPacks<'a> {
    #[serde(rename = "Disabled")]
    #[serde(borrow)]
    disabled: Vec<Cow<'a, str>>,

    #[serde(rename = "Enabled")]
    #[serde(borrow)]
    enabled: Vec<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewLevelDatDifficultySettings<'a> {
    difficulty: Cow<'a, str>,
    hardcore: bool,
    locked: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewLevelDatSpawn {
    dimension: String,
    pitch: f32,
    yaw: f32,
    pos: IntArray,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewLevelDatVersionInfo {
    #[serde(rename = "Id")]
    id: i32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Series")]
    series: String,

    #[serde(rename = "Snapshot")]
    snapshot: bool,
}
