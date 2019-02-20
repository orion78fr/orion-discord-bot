extern crate discord;

use std::env;

use discord::Discord;
use discord::model::Event;

fn main() {
    // Authenticate with discord
    let token = &env::var("DISCORD_TOKEN").expect("Expected token in DISCORD_TOKEN");
    let discord = Discord::from_bot_token(token).expect("Login failed, please check your token");

    // Get websocket connection
    let (mut connection, _) = discord.connect().expect("Connect failed");

    println!("Ready to receive messages !");

    // Loop through the events
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id,
                                                 "It works !", "", false);
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}
