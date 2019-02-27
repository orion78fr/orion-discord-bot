use std::collections::HashMap;
use std::time::SystemTime;

use serenity::model::gateway::Presence;
use serenity::model::id::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    last_update: u64,
    points: HashMap<UserId, (u64, u64)>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            last_update: cur_ts(),
            points: HashMap::new(),
        }
    }

    pub fn new_player(&mut self, user_id: &UserId) {
        if !self.points.contains_key(user_id) {
            self.points.insert(user_id.clone(), (100, 1));
        }
    }

    pub fn update_presence(&mut self, _presence: &Presence) {}
}

fn cur_ts() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
