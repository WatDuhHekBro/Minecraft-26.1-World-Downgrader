use fastnbt::IntArray;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::dat::level_common::{LevelDatDataPacks, LevelDatSpawn, LevelDatVersionInfo};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewLevelDat<'a> {
    #[serde(rename = "Data")]
    #[serde(borrow)]
    pub data: NewLevelDatData<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewLevelDatData<'a> {
    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,

    #[serde(rename = "DataPacks")]
    #[serde(borrow)]
    pub datapacks: Option<LevelDatDataPacks<'a>>,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(borrow)]
    pub difficulty_settings: NewLevelDatDifficultySettings<'a>,

    #[serde(borrow)]
    pub enabled_features: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "GameType")]
    pub game_type: i32,

    pub initialized: bool,

    #[serde(rename = "LastPlayed")]
    pub last_played: i64,

    #[serde(rename = "LevelName")]
    #[serde(borrow)]
    pub level_name: Cow<'a, str>,

    pub singleplayer_uuid: Option<IntArray>,

    #[serde(rename = "ServerBrands")]
    #[serde(borrow)]
    pub server_brands: Option<Vec<Cow<'a, str>>>,

    pub spawn: LevelDatSpawn,

    #[serde(rename = "Time")]
    pub time: i64,

    pub version: i32,

    #[serde(rename = "Version")]
    pub version_info: LevelDatVersionInfo,

    #[serde(rename = "WasModded")]
    pub was_modded: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewLevelDatDifficultySettings<'a> {
    pub difficulty: Cow<'a, str>,
    pub hardcore: bool,
    pub locked: bool,
}

impl<'a> NewLevelDatDifficultySettings<'a> {
    pub fn get_difficulty_integer(&self) -> i8 {
        match self.difficulty {
            Cow::Borrowed("peaceful") => 0,
            Cow::Borrowed("easy") => 1,
            Cow::Borrowed("normal") => 2,
            Cow::Borrowed("hard") => 3,
            _ => panic!(
                "{} is not a valid difficulty setting in level.dat!",
                self.difficulty
            ),
        }
    }
}
