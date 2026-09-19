///////////////
// IMPORTANT //
///////////////
/// WorldGenSettings is the KEY to why your world(s) will not load even in safe mode.
/// Get this property into the downgraded level.dat and you will actually be able to load the world in 1.21.11.
/// Same reason why any mods that adds dimensions prevents you from booting it up in safe mode either.
use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWorldGenSettings {
    pub data: NewWorldGenSettingsData,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewWorldGenSettingsData {
    pub bonus_chest: bool,
    pub generate_structures: bool,
    pub seed: i64,
    pub dimensions: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OldWorldGenSettings {
    pub bonus_chest: bool,
    pub generate_features: bool,
    pub seed: i64,
    pub dimensions: Value,
}

impl OldWorldGenSettings {
    pub fn from(worldgen: &NewWorldGenSettings) -> OldWorldGenSettings {
        let NewWorldGenSettings {
            data,
            data_version: _,
        } = worldgen;
        let NewWorldGenSettingsData {
            bonus_chest,
            generate_structures,
            seed,
            dimensions,
        } = data;

        OldWorldGenSettings {
            bonus_chest: *bonus_chest,
            generate_features: *generate_structures,
            seed: *seed,
            dimensions: dimensions.clone(),
        }
    }
}
