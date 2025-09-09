mod handlers;
mod utils;

#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(all(feature = "bot", feature = "self-bot")) {
        panic!(
            "Feature 'bot' is incompatible with feature 'self-bot'. Please disable one of these features."
        );
    }
    dotenvy::dotenv()?;

    let token = dotenvy::var("TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(handlers::CustomHandler)
        .await;

    Ok(client.unwrap().start().await?)
}
