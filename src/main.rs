mod handlers;
mod utils;

#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    #[cfg(feature = "bot")]
    let token = dotenvy::var("TOKEN")?;
    #[cfg(feature = "self-bot")]
    let token = dotenvy::var("USER_TOKEN")?;

    let intents = serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .event_handler(handlers::CustomHandler)
        .await
        .unwrap();

    Ok(client.start().await?)
}
