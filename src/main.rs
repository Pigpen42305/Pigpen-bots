mod commands;
#[cfg(debug_assertions)]
mod env_utils;
mod handler;
mod logging;
mod message_utils;
mod parser;

use std::env::var;

use serenity::prelude::*;

const PREFIX: &str = "/bot ";

const _: () = const {
    const END: u8 = PREFIX.as_bytes()[PREFIX.len() - 1];
    assert!(
        (END == b' ') || (END == b'\n'),
        "The PREFIX must end with a space or newline character!"
    )
};

const MAX_MESSAGE_LEN: usize = 2000;

#[tokio::main]
async fn main() {
    let Ok(_guard) = logging::setup_logging() else {
        panic!("Failed to setup the logger!");
    };

    #[cfg(debug_assertions)]
    if !env_utils::setup_env().unwrap_or(false) {
        panic!("Expected the DISCORD_TOKEN in the environment...")
    }
    // Login with a bot token from the environment
    let token = var("DISCORD_TOKEN").expect("Expected the DISCORD_TOKEN environment variable...");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
