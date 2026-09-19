use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::dat::{
    level_common::{LevelDatDataPacks, LevelDatSpawn, LevelDatVersionInfo},
    level_new::{NewLevelDat, NewLevelDatData},
    weather::{NewWeather, NewWeatherData},
    world_gen_settings::{NewWorldGenSettings, OldWorldGenSettings},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDat<'a> {
    #[serde(rename = "Data")]
    #[serde(borrow)]
    data: OldLevelDatData<'a>,
}

// Option doesn't matter too much here because you're only writing.
#[derive(Deserialize, Serialize, Debug)]
struct OldLevelDatData<'a> {
    #[serde(rename = "allowCommands")]
    allow_commands: bool,

    #[serde(rename = "BorderCenterX")]
    border_center_x: f64,

    #[serde(rename = "BorderCenterZ")]
    border_center_z: f64,

    #[serde(rename = "BorderDamagePerBlock")]
    border_damage_per_block: f64,

    #[serde(rename = "BorderSize")]
    border_size: f64,

    #[serde(rename = "BorderSafeZone")]
    border_safe_zone: f64,

    #[serde(rename = "BorderSizeLerpTarget")]
    border_size_lerp_target: f64,

    #[serde(rename = "BorderSizeLerpTime")]
    border_size_lerp_time: i64,

    #[serde(rename = "BorderWarningBlocks")]
    border_warning_blocks: f64,

    #[serde(rename = "BorderWarningTime")]
    border_warning_time: f64,

    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,

    //#[serde(rename = "CustomBossEvents")]
    //custom_boss_events: serde_json::Value,
    #[serde(rename = "DataPacks")]
    #[serde(borrow)]
    datapacks: Option<LevelDatDataPacks<'a>>,

    #[serde(rename = "DataVersion")]
    data_version: i32,

    //#[serde(rename = "DayTime")]
    //day_time: i64,
    #[serde(rename = "Difficulty")]
    difficulty: i8,

    #[serde(rename = "DifficultyLocked")]
    difficulty_locked: bool,

    //#[serde(rename = "DimensionData")]
    //dimension_data: serde_json::Value,
    #[serde(borrow)]
    enabled_features: Option<Vec<Cow<'a, str>>>,

    //#[serde(rename = "GameRules")]
    // NOTE: It's just "game_rules" according to my 1.21.11 level.dat.
    //game_rules: Value,
    #[serde(rename = "GameType")]
    game_type: i32,

    hardcore: bool,

    initialized: bool,

    #[serde(rename = "LastPlayed")]
    last_played: i64,

    #[serde(rename = "LevelName")]
    #[serde(borrow)]
    level_name: Cow<'a, str>,

    #[serde(rename = "MapFeatures")]
    map_features: bool,

    raining: bool,

    #[serde(rename = "rainTime")]
    rain_time: i32,

    #[serde(rename = "RandomSeed")]
    random_seed: i64,

    #[serde(rename = "ServerBrands")]
    #[serde(borrow)]
    server_brands: Option<Vec<Cow<'a, str>>>,

    // NOTE: Appears in my 1.21.11 level.dat but not the old wiki
    spawn: LevelDatSpawn,

    // NOTE: But then again, it's better to have it than not.
    // The game will ignore any irrelevant fields (but can't make up ones that don't exist).
    #[serde(rename = "SpawnX")]
    spawn_x: i32,
    #[serde(rename = "SpawnY")]
    spawn_y: i32,
    #[serde(rename = "SpawnZ")]
    spawn_z: i32,

    thundering: bool,

    #[serde(rename = "thunderTime")]
    thunder_time: i32,

    #[serde(rename = "Time")]
    time: i64,

    version: i32,

    #[serde(rename = "Version")]
    version_info: LevelDatVersionInfo,

    /*#[serde(rename = "WanderingTraderId")]
    wandering_trader_id: Option<IntArray>,

    #[serde(rename = "WanderingTraderSpawnChance")]
    wandering_trader_spawn_chance: i32,

    #[serde(rename = "WanderingTraderSpawnDelay")]
    wandering_trader_spawn_delay: i32,*/
    #[serde(rename = "WasModded")]
    was_modded: bool,

    #[serde(rename = "WorldGenSettings")]
    world_gen_settings: OldWorldGenSettings,
}

pub fn convert_new_leveldat_to_old_leveldat(
    leveldat: NewLevelDat,
    weather: NewWeather,
    worldgen: NewWorldGenSettings,
) -> OldLevelDat {
    let NewLevelDatData {
        allow_commands,
        datapacks,
        data_version,
        difficulty_settings,
        enabled_features,
        game_type,
        initialized,
        last_played,
        level_name,
        singleplayer_uuid: _,
        server_brands,
        spawn,
        time,
        version,
        version_info: _,
        was_modded,
    } = leveldat.data;

    let NewWeatherData {
        clear_weather_time,
        raining,
        rain_time,
        thundering,
        thunder_time,
    } = weather.data;

    OldLevelDat {
        data: OldLevelDatData {
            allow_commands,
            border_center_x: 0.0,
            border_center_z: 0.0,
            border_damage_per_block: 0.0,
            border_size: 0.0,
            border_safe_zone: 0.0,
            border_size_lerp_target: 0.0,
            border_size_lerp_time: 0,
            border_warning_blocks: 0.0,
            border_warning_time: 0.0,
            clear_weather_time,
            datapacks,
            data_version,
            //day_time: (),
            difficulty: difficulty_settings.get_difficulty_integer(),
            difficulty_locked: difficulty_settings.locked,
            enabled_features,
            //game_rules: (),
            game_type,
            hardcore: difficulty_settings.hardcore,
            initialized,
            last_played,
            level_name,
            map_features: worldgen.data.generate_structures,
            raining,
            rain_time,
            random_seed: worldgen.data.seed,
            server_brands,
            spawn,
            spawn_x: 0,
            spawn_y: 320,
            spawn_z: 0,
            thundering,
            thunder_time,
            time,
            version,
            version_info: LevelDatVersionInfo {
                id: 4671,
                name: "1.21.11".into(),
                series: "main".into(),
                snapshot: false,
            },
            //wandering_trader_id: (),
            //wandering_trader_spawn_chance: (),
            //wandering_trader_spawn_delay: (),
            was_modded,
            world_gen_settings: OldWorldGenSettings::from(worldgen),
        },
    }
}
