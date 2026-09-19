mod convert_level;
mod custom_boss_events;
mod ender_dragon_fight;
mod game_rules;
mod level_common;
mod level_new;
mod level_old;
mod scheduled_events;
mod wandering_trader;
mod weather;
mod world_clocks;
mod world_gen_settings;

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::dat::level_new::NewLevelDat;

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

pub fn write_leveldat(absolute_path: &Path) {
    //
}

/*pub fn asdf() {
    let file = File::open("/home/watduhhekbro/downloads/galarov/GALAROV_TEST/level.dat").unwrap();
    let filew =
        File::create("/home/watduhhekbro/downloads/galarov/GALAROV_TEST/level++.dat").unwrap();

    // Player dat files are compressed with GZip.
    let mut decoder = GzDecoder::new(file);
    let mut data = vec![];
    decoder.read_to_end(&mut data).unwrap();

    let player = fastnbt::from_bytes::<OldLevelDat>(data.as_slice()).unwrap();

    println!("{:#?}", player);

    let new_bytes = fastnbt::to_bytes(&player).unwrap();
    let mut encoder = GzEncoder::new(filew, Compression::fast());
    encoder.write_all(&new_bytes).unwrap();
}*/
