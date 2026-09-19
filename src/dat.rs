pub mod convert_level;
pub mod custom_boss_events;
pub mod ender_dragon_fight;
pub mod game_rules;
pub mod level_common;
pub mod level_new;
pub mod level_old;
pub mod scheduled_events;
pub mod wandering_trader;
pub mod weather;
pub mod world_clocks;
pub mod world_gen_settings;

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::dat::{
    level_new::NewLevelDat, level_old::OldLevelDat, weather::NewWeather,
    world_gen_settings::NewWorldGenSettings,
};

pub fn read_gzipped_bytes(absolute_path: &Path) -> Vec<u8> {
    let file = File::open(absolute_path).unwrap();

    // .dat files are compressed with GZip
    let mut decoder = GzDecoder::new(file);
    let mut data = vec![];
    decoder.read_to_end(&mut data).unwrap();

    data
}

pub fn write_gzipped_bytes(absolute_path: &Path, bytes: &Vec<u8>) {
    let file = File::create(absolute_path).unwrap();
    let mut encoder = GzEncoder::new(file, Compression::fast());
    encoder.write_all(bytes).unwrap();
}

pub fn read_leveldat(absolute_path: &Path) -> NewLevelDat {
    let bytes = read_gzipped_bytes(absolute_path);
    let leveldat = fastnbt::from_bytes::<NewLevelDat>(&bytes).unwrap();
    leveldat
}

pub fn write_leveldat(absolute_path: &Path, leveldat: OldLevelDat) {
    let bytes = fastnbt::to_bytes(&leveldat).unwrap();
    write_gzipped_bytes(absolute_path, &bytes);
}

pub fn read_weather(absolute_path: &Path) -> NewWeather {
    let bytes = read_gzipped_bytes(absolute_path);
    let data = fastnbt::from_bytes::<NewWeather>(&bytes).unwrap();
    data
}

pub fn read_worldgen(absolute_path: &Path) -> NewWorldGenSettings {
    let bytes = read_gzipped_bytes(absolute_path);
    let data = fastnbt::from_bytes::<NewWorldGenSettings>(&bytes).unwrap();
    data
}
