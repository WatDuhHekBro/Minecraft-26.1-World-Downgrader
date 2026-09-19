/*
use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatCustomBossEvents<'a>(
    #[serde(borrow)]
    pub HashMap<Cow<'a, str>, OldLevelDatCustomBossEvent<'a>>,
);

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatCustomBossEvent<'a> {
    #[serde(borrow)]
    pub players: Vec<OldLevelDatCustomBossEventPlayer>,

    #[serde(borrow)]
    pub color: Cow<'a, str>,

    #[serde(rename = "CreateWorldFog")]
    pub create_world_fog: bool,

    #[serde(rename = "DarkenScreen")]
    pub darken_screen: bool,

    #[serde(rename = "Max")]
    pub max: i32,

    #[serde(rename = "Value")]
    pub value: i32,

    #[serde(borrow)]
    pub name: Cow<'a, str>,

    #[serde(borrow)]
    pub overlay: Cow<'a, str>,

    #[serde(rename = "PlayBossMusic")]
    pub play_boss_music: bool,

    pub visible: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatCustomBossEventPlayer {
    #[serde(rename = "UUID")]
    pub uuid: IntArray,
}
*/
