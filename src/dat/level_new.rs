use fastnbt::IntArray;
use serde::{Deserialize, Serialize};

use crate::dat::level_common::{LevelDatDataPacks, LevelDatSpawn, LevelDatVersionInfo};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLevelDat {
    #[serde(rename = "Data")]
    pub data: NewLevelDatData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLevelDatData {
    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,

    #[serde(rename = "DataPacks")]
    pub datapacks: Option<LevelDatDataPacks>,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    pub difficulty_settings: NewLevelDatDifficultySettings,

    pub enabled_features: Option<Vec<String>>,

    #[serde(rename = "GameType")]
    pub game_type: i32,

    pub initialized: bool,

    #[serde(rename = "LastPlayed")]
    pub last_played: i64,

    #[serde(rename = "LevelName")]
    pub level_name: String,

    pub singleplayer_uuid: Option<IntArray>,

    #[serde(rename = "ServerBrands")]
    pub server_brands: Option<Vec<String>>,

    pub spawn: LevelDatSpawn,

    #[serde(rename = "Time")]
    pub time: i64,

    pub version: i32,

    #[serde(rename = "Version")]
    pub version_info: LevelDatVersionInfo,

    #[serde(rename = "WasModded")]
    pub was_modded: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLevelDatDifficultySettings {
    pub difficulty: String,
    pub hardcore: bool,
    pub locked: bool,
}

impl NewLevelDatDifficultySettings {
    pub fn get_difficulty_integer(&self) -> i8 {
        match self.difficulty.as_str() {
            "peaceful" => 0,
            "easy" => 1,
            "normal" => 2,
            "hard" => 3,
            _ => panic!(
                "{} is not a valid difficulty setting in level.dat!",
                self.difficulty
            ),
        }
    }
}
