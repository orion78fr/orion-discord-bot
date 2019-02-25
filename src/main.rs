extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serenity;

mod parser;

use crate::parser::parse_message;

use std::{env, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    model::{channel::Message, channel::ReactionType, gateway::Ready, id::UserId},
    prelude::*,
    utils::MessageBuilder,
};

struct CurrentUser;

impl TypeMapKey for CurrentUser {
    type Value = (UserId, String, u16);
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let current_user_id;

        {
            let data = ctx.data.lock();
            current_user_id = match data.get::<CurrentUser>() {
                Some((id, _,_)) => id.clone(),
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
        let parsed = parse_message(&safe_msg_content);

        if parsed.is_err() {
            // Error while parsing, mark the message and return
            if let Err(why) = msg.react(ReactionType::Unicode(String::from("❌"))) {
                println!("Error reacting to message : {:?}", why);
            }
            return;
        } else {
            if let Err(why) = msg.react(ReactionType::Unicode(String::from("✅"))) {
                println!("Error reacting to message : {:?}", why);
            }
        }

        let mut response = MessageBuilder::new()
            .push("Message ")
            .push_mono_safe(&msg.content_safe())
            .push(" parsed to : \n");

        for elem in parsed.unwrap() {
            response = response.push_codeblock_safe(elem, None).push("\n");
        }

        if let Err(why) = msg.channel_id.say(response.build()) {
            println!("Error sending message : {:?}", why);
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        let user = ready.user;

        println!("{}#{} is connected and ready to receive messages !",
                 user.name, user.discriminator);

        ctx.data.lock().insert::<CurrentUser>((user.id, user.name, user.discriminator));
    }
}

fn main() {
    // Authenticate with discord
    let token = &env::var("DISCORD_TOKEN").expect("Expected token in DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Start the bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
