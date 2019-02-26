use std::fs::File;

use ron::de::from_reader;
use ron::ser::to_string;

use model::GameData;
use std::io::Write;

pub mod model;

pub fn save(path: &str, data: &GameData) -> Result<(), String> {
    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(why) => return Err(why.to_string())
    };

    match f.write(to_string(data).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.to_string())
    }
}

pub fn load(path: &str) -> Result<GameData, String> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(why) => return Err(why.to_string())
    };

    match from_reader(f) {
        Ok(r) => Ok(r),
        Err(why) => Err(why.to_string())
    }
}