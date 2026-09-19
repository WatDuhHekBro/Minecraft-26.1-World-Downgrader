/*
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatDimensionData {
    #[serde(rename = "1")]
    pub end: OldLevelDatDimensionDataEnd,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatDimensionDataEnd {
    #[serde(rename = "DragonFight")]
    pub dragon_fight: Option<OldLevelDatDragonFight>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatDragonFight {
    #[serde(rename = "ExitPortalLocation")]
    pub exit_portal_location: OldLevelDatDragonFightExitPortalLocation,

    #[serde(rename = "Gateways")]
    pub gateways: Vec<i32>,

    #[serde(rename = "DragonKilled")]
    pub dragon_killed: bool,

    #[serde(rename = "DragonUUIDLeast")]
    pub dragon_uuid_least: i64,

    #[serde(rename = "DragonUUIDMost")]
    pub dragon_uuid_most: i64,

    #[serde(rename = "PreviouslyKilled")]
    pub previously_killed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDatDragonFightExitPortalLocation {
    #[serde(rename = "X")]
    pub x: i8,

    #[serde(rename = "Y")]
    pub y: i8,

    #[serde(rename = "Z")]
    pub z: i8,
}
*/
