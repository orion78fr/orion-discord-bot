use serenity::model::gateway::Presence;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    last_update: u64
}

impl GameData {
    pub fn new() -> GameData {
        GameData { last_update: 0 }
    }

    pub fn update_presence(&mut self, _presence: &Presence) {
        self.last_update = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }
}