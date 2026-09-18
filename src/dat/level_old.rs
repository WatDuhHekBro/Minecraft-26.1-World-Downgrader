use fastnbt::{ByteArray, IntArray, LongArray};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug)]
pub struct OldLevelDat<'a> {
    #[serde(rename = "Data")]
    #[serde(borrow)]
    data: OldLevelDatData<'a>,
}

/*
[Double] BorderCenterX: Center of the world border on the X coordinate. Defaults to 0.
[Double] BorderCenterZ: Center of the world border on the Z coordinate. Defaults to 0.
[Double] BorderDamagePerBlock: Defaults to 0.2.
[Double] BorderSize: Width and length of the border of the world. Defaults to 60000000.
[Double] BorderSafeZone: Defaults to 5.
[Double] BorderSizeLerpTarget: Defaults to 60000000.
[Long] BorderSizeLerpTime: Defaults to 0.
[Double] BorderWarningBlocks: Defaults to 5.
[Double] BorderWarningTime: Defaults to 15.
[Int] clearWeatherTime: The number of ticks until "clear weather" has ended.

[NBT Compound / JSON Object] CustomBossEvents: A collection of bossbars.

[Long] DayTime: The time of day. 0 is sunrise, 6000 is mid day, 12000 is sunset, 18000 is mid night, 24000 is the next day's 0. This value keeps counting past 24000 and does not reset to 0.
[Byte] Difficulty: The current difficulty setting. 0 is Peaceful, 1 is Easy, 2 is Normal, and 3 is Hard. Defaults to 2.
[Boolean] DifficultyLocked: 1 or 0 (true/false) - True if the difficulty has been locked. Defaults to 0.

[NBT Compound / JSON Object] DimensionData: This tag contains level data specific to certain dimensions.

[NBT Compound / JSON Object] GameRules: The gamerules used in the world.

[NBT Compound / JSON Object] WorldGenSettings: Used in 1.16, the generation settings for each dimension.

[String] generatorName: Used in 1.15 and below. The name of the generator; default, flat, largeBiomes, amplified, buffet, debug_all_block_states or default_1_1. Not case sensitive, but always written in the case here. The last one can exist only if the file was edited. It is a variation of the default generation. It can also be customized if it is a customized world from before 1.13. In this case the world becomes default if opened using 1.13 or newer.
[NBT Compound / JSON Object] generatorOptions: Used in 1.15 and below. Used in buffet, superflat, and old customized worlds. Format below.
[Int] generatorVersion: Used in 1.15 and below. The version of the level generator. The effects of changing this are unknown, but values other than 0 have been observed.
[Boolean] hardcore: 1 or 0 (true/false) - If true, the player respawns in Spectator on death in singleplayer. Affects all three game modes.

[Boolean] MapFeatures: 1 or 0 (true/false) - true if the map generator should place structures such as villages, strongholds, and mineshafts. Defaults to 1. Always 1 if the world type is Customized.

[Boolean] raining: 1 or 0 (true/false) - true if the level is currently experiencing rain, snow, and cloud cover.
[Int] rainTime: The number of ticks before "raining" is toggled and this value gets set to another random value.
[Long] RandomSeed: The random level seed used to generate consistent terrain.
[Long] SizeOnDisk: The estimated size in bytes of the level. Currently not modified or used by Minecraft, but was previously.
[Int] SpawnX: The X coordinate of the world spawn.
[Int] SpawnY: The Y coordinate of the world spawn.
[Int] SpawnZ: The Z coordinate of the world spawn.
[Boolean] thundering: 1 or 0 (true/false) - If "raining" is true : true if the rain/snow/cloud cover is a lightning storm and dark enough for mobs to spawn under the sky. If "raining" is false, this has no effect.
[Int] thunderTime: The number of ticks before "thundering" is toggled and this value gets set to another random value.

[Int Array] WanderingTraderId: The UUID of the current wandering trader in the world saved as four ints.
[Int] WanderingTraderSpawnChance: The current chance of the wandering trader spawning next attempt; this value is the percentage and is divided by 10 when loaded by the game, for example a value of 50 means 5.0% chance.
[Int] WanderingTraderSpawnDelay: The amount of ticks until another wandering trader is attempted to spawn.
*/

#[derive(Deserialize, Serialize, Debug)]
struct OldLevelDatData<'a> {
    #[serde(rename = "allowCommands")]
    allow_commands: bool,

    #[serde(rename = "DataPacks")]
    #[serde(borrow)]
    datapacks: Option<OldLevelDatDataPacks<'a>>,

    #[serde(rename = "DataVersion")]
    data_version: i32,

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

    #[serde(rename = "Time")]
    time: i64,

    version: i32,

    #[serde(rename = "Version")]
    version_info: OldLevelDatVersionInfo,

    #[serde(rename = "WasModded")]
    was_modded: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct OldLevelDatDataPacks<'a> {
    #[serde(rename = "Disabled")]
    #[serde(borrow)]
    disabled: Vec<Cow<'a, str>>,

    #[serde(rename = "Enabled")]
    #[serde(borrow)]
    enabled: Vec<Cow<'a, str>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct OldLevelDatVersionInfo {
    #[serde(rename = "Id")]
    id: i32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Series")]
    series: String,

    #[serde(rename = "Snapshot")]
    snapshot: bool,
}
