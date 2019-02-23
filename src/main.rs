extern crate serenity;

use std::{env, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::MessageBuilder,
};

struct CurrentUserId;

impl TypeMapKey for CurrentUserId {
    type Value = UserId;
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
            current_user_id = match data.get::<CurrentUserId>() {
                Some(id) => id.clone(),
                None => return
            }
        }

        if msg.author.id == current_user_id {
            // Do not react to our own messages, EVER (or we could be stuck in a loop)
            return;
        }

        if !msg.mentions_user_id(current_user_id) {
            // We only react on mentions to us, thus we don't have any incompatibility to other bots
            return;
        }

        // Just answer to the user
        let channel = match msg.channel_id.to_channel() {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {:?}", why);

                return;
            }
        };

        let response = MessageBuilder::new()
            .push("User ")
            .mention(&msg.author)
            .push(" mentioned me in the ")
            .mention(&channel)
            .push(" channel of the guild \"")
            .push_safe(&channel.guild().unwrap().read().guild().unwrap().read().name)
            .push("\" with the message ")
            .push_mono_safe(&msg.content_safe())
            .build();

        if let Err(why) = msg.channel_id.say(response) {
            println!("Error sending message : {:?}", why);
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{}#{} is connected and ready to receive messages !",
                 ready.user.name, ready.user.discriminator);

        ctx.data.lock().insert::<CurrentUserId>(ready.user.id);
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
