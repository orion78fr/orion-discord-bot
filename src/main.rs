extern crate serenity;

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!test" {
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
                .push(" used the 'test' command in the ")
                .mention(&channel)
                .push(" channel of the guild ")
                .push(&channel.guild().unwrap().read().guild().unwrap().read().name)
                .build();

            if let Err(why) = msg.channel_id.say(response) {
                println!("Error sending message : {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{}#{} is connected and ready to receive messages !",
                 ready.user.name, ready.user.discriminator);
    }
}
fn main() {
    // Authenticate with discord
    let token = &env::var("DISCORD_TOKEN").expect("Expected token in DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    // Start the bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
