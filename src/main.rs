pub mod commands;
mod utils;

use commands::*;
use serenity::all as serenity;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let token = dotenvy::var("TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    Ok(client.unwrap().start().await?)
}
