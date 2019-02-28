use std::collections::HashMap;
use std::time::SystemTime;

use serenity::model::channel::ReactionType;
use serenity::model::gateway::Presence;

use crate::Answer;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    last_update: u64,
    points: HashMap<u64, (u64, u64)>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            last_update: cur_ts(),
            points: HashMap::new(),
        }
    }

    pub fn new_player(&mut self, user_id: u64) -> Answer {
        if !self.points.contains_key(&user_id) {
            self.points.insert(user_id, (100, 1));
            return Answer::Reaction(ReactionType::Unicode(String::from("✅")));
        }
        Answer::Reaction(ReactionType::Unicode(String::from("⚠")))
    }

    pub fn get_status(&mut self, user_id: u64) -> Answer {
        if !self.points.contains_key(&user_id) {
            return Answer::Message(format!("User {} not found", user_id));
        }

        let (points, speed) = self.points.get(&user_id).unwrap();

        Answer::Message(format!("{} points (+{}/s)", points, speed))
    }

    pub fn update_presence(&mut self, _presence: &Presence) {}

    pub fn update(&mut self) {
        self.points.iter_mut()
            .for_each(|(_, (points, speed))| *points += *speed);
    }
}

fn cur_ts() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
