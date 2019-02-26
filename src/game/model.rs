use serenity::model::gateway::Presence;

pub struct GameData {
    last_save: u64
}

impl GameData {
    pub fn new() -> GameData {
        GameData { last_save: 0 }
    }

    pub fn update_presence(&mut self, presence: &Presence) {}
}