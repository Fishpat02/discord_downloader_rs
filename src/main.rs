mod handlers;
mod rest;
mod utils;

use serde::Deserialize;
#[cfg(feature = "bot")]
use serenity::all as serenity;
#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Deserialize)]
struct ChannelConfig {
    id: u64,
    save_location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    channels: Vec<ChannelConfig>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    use serenity::{ClientBuilder, GatewayIntents};

    dotenvy::dotenv()?;

    #[cfg(feature = "bot")]
    let token = dotenvy::var("TOKEN")?;
    #[cfg(feature = "self-bot")]
    let token = dotenvy::var("USER_TOKEN")?;

    #[cfg(feature = "self-bot")]
    println!(concat!(
        "Be aware that running a bot application on a user account is againt Discord TOS.\n",
        "Proceed at your own risk!!"
    ));

    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(token, intents)
        .event_handler(handlers::CustomHandler)
        .await
        .unwrap();

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Couldn't register ctrl+c");
        shard_manager.shutdown_all().await;
    });

    Ok(client.start().await?)
}
