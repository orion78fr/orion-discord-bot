extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serenity;

use std::{env, sync::Arc};
use std::thread;
use std::time::Duration;

use serenity::{
    client::bridge::gateway::ShardManager,
    model::{channel::Message, channel::ReactionType, gateway::Ready, id::UserId},
    prelude::*,
};
use serenity::model::event::PresenceUpdateEvent;
use serenity::utils::MessageBuilder;

use game::model::GameData;
use parser::parse_message;

use crate::game::save;

mod parser;
mod game;

struct CurrentUser;

impl TypeMapKey for CurrentUser {
    type Value = (UserId, String, u16);
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct GameDataContainer;

impl TypeMapKey for GameDataContainer {
    type Value = Arc<Mutex<GameData>>;
}

struct Handler;

pub enum Answer {
    Message(String),
    Reaction(ReactionType),
    None,
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let current_user_id;
        let current_user_name;
        let current_user_discriminator;

        {
            let data = ctx.data.lock();
            match data.get::<CurrentUser>() {
                Some((id, name, disc)) => {
                    current_user_id = id.clone();
                    current_user_name = name.clone();
                    current_user_discriminator = disc.clone();
                }
                None => return
            }
        }

        if msg.author.id == current_user_id {
            // Do not react to our own messages, EVER (or we could be stuck in a loop)
            return;
        }

        if !msg.mentions_user_id(current_user_id) && !msg.is_private() {
            // We only react on mentions to us, thus we don't have any incompatibility to other bots
            // Or direct messages
            return;
        }

        // Parse the args
        let safe_msg_content = msg.content_safe();
        let parsed = parse_message(&safe_msg_content,
                                   // Remove the mention from the args
                                   Some(format!("@{}#{}", current_user_name, current_user_discriminator)));

        if parsed.is_err() {
            // Error while parsing, mark the message and return
            println!("Error parsing : {}", parsed.unwrap_err());
            if let Err(why) = msg.react(ReactionType::Unicode(String::from("💥"))) {
                println!("Error reacting to message : {:?}", why);
            }
            return;
        }

        let parsed = parsed.unwrap();

        let mut data = ctx.data.lock();
        let mut game_data = data.get_mut::<GameDataContainer>().unwrap().lock();

        let answer = match parsed[0] {
            "join" => game_data.new_player(msg.author.id.0),
            "status" => game_data.get_status(msg.author.id.0),
            _ => Answer::Reaction(ReactionType::Unicode(String::from("❌")))
        };

        match answer {
            Answer::Message(m) => {
                if msg.is_private() {
                    if let Err(why) = msg.channel_id.say(m) {
                        println!("Error answering to message : {:?}", why);
                    };
                } else {
                    if let Err(why) = msg.channel_id.say(MessageBuilder::new()
                        .mention(&msg.author)
                        .push(" ")
                        .push(m)
                        .build()) {
                        println!("Error answering to message : {:?}", why);
                    };
                }
            }
            Answer::Reaction(r) => {
                if let Err(why) = msg.react(r) {
                    println!("Error reacting to message : {:?}", why);
                };
            }
            Answer::None => {}
        };
    }

    fn presence_update(&self, ctx: Context, new_data: PresenceUpdateEvent) {
        let mut data = ctx.data.lock();
        let mut game_data = data.get_mut::<GameDataContainer>().unwrap().lock();

        game_data.update_presence(&new_data.presence);
        match save(GAME_DATA, &game_data) {
            Err(why) => println!("Cannot save game data ! {}", why),
            _ => {}
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        let user = ready.user;

        println!("{}#{} is connected and ready to receive messages !",
                 user.name, user.discriminator);

        ctx.data.lock().insert::<CurrentUser>((user.id, user.name, user.discriminator));
    }
}

const GAME_DATA: &str = "./data.ron";

fn main() {
    // Authenticate with discord
    let token = &env::var("DISCORD_TOKEN").expect("Expected token in DISCORD_TOKEN");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    let game_data = Arc::new(Mutex::new(match game::load(GAME_DATA) {
        Ok(data) => dbg!(data),
        Err(why) => {
            println!("Cannot load data, creating new : {}", why);
            GameData::new()
        }
    }));

    {
        let mut data = client.data.lock();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<GameDataContainer>(Arc::clone(&game_data));
    }

    thread::spawn(move || {
        loop {
            {
                game_data.lock().update();
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Start the bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
