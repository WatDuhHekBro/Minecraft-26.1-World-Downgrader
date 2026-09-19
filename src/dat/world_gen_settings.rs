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

#[derive(Deserialize, Serialize, Debug)]
pub struct OldWorldGenSettings {
    pub data: OldWorldGenSettingsData,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OldWorldGenSettingsData {
    pub bonus_chest: bool,
    pub generate_features: bool,
    pub seed: i64,
    pub dimensions: Value,
}

impl OldWorldGenSettings {
    pub fn from(worldgen: NewWorldGenSettings) -> OldWorldGenSettings {
        let NewWorldGenSettings { data, data_version } = worldgen;
        let NewWorldGenSettingsData {
            bonus_chest,
            generate_structures,
            seed,
            dimensions,
        } = data;

        OldWorldGenSettings {
            data: OldWorldGenSettingsData {
                bonus_chest,
                generate_features: generate_structures,
                seed,
                dimensions,
            },
            data_version,
        }
    }
}
