use model::GameData;

pub mod model;

pub fn save<'a>(path: &'a str, data: &GameData) -> Result<(), &'a str> {
    Err("Not Implemented")
}

pub fn load(path: &str) -> Result<GameData, &str> {
    Err("Not Implemented")
}