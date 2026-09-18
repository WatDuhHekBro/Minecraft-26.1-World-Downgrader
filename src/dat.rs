mod level_new;
mod level_old;

use fastnbt::{error::Result, from_bytes};
use flate2::read::GzDecoder;
//use level_new::NewLevelDat;
use level_old::OldLevelDat;
use std::{fs::File, io::Read};

pub fn asdf() {
    let file = File::open("/home/watduhhekbro/downloads/galarov/GALAROV_TEST/level.dat").unwrap();

    // Player dat files are compressed with GZip.
    let mut decoder = GzDecoder::new(file);
    let mut data = vec![];
    decoder.read_to_end(&mut data).unwrap();

    let player: Result<OldLevelDat> = from_bytes(data.as_slice());

    println!("{:#?}", player);
}
