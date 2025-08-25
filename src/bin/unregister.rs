use discord_scraper::*;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let token = dotenvy::var("TOKEN")?;
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .setup(|_ctx, _ready, _framework: &poise::Framework<Data, Error>| {
            Box::pin(async move { Ok(Data {}) })
        })
        .options(poise::FrameworkOptions::default())
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .application_id(serenity::ApplicationId::new(
            dotenvy::var("CLIENT")?.parse::<u64>()?,
        ))
        .framework(framework)
        .await?;

    let commands = client.http.get_global_commands().await?;
    for cmd in commands {
        client.http.delete_global_command(cmd.id).await?;
    }

    Ok(())
}
