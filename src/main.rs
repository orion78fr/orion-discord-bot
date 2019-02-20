extern crate serenity;

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!test" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say("It works !") {
                println!("Error sending message : {:?}", why);
            }
        } else if message.content == "!exit" {
            println!("Received exit command, exiting...");
            ctx.quit();
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready to receive messages !", ready.user.name);
    }
}

fn main() {
    // Authenticate with discord
    let token = &env::var("DISCORD_TOKEN").expect("Expected token in DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
