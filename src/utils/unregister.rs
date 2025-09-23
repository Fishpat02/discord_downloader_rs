#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

use crate::Error;

pub async fn unregister() -> Result<(), Error> {
    use serenity::{ApplicationId, ClientBuilder, GatewayIntents, Http};

    dotenvy::dotenv()?;

    let token = dotenvy::var("TOKEN")?;
    let intents = GatewayIntents::all();

    let app_id = ApplicationId::new(dotenvy::var("CLIENT")?.parse::<u64>()?);
    let http = Http::new(&token);
    http.set_application_id(app_id);

    let client = ClientBuilder::new_with_http(http, intents).await?;

    let commands = client.http.get_global_commands().await?;
    for cmd in commands {
        client.http.delete_global_command(cmd.id).await?;
    }

    Ok(())
}
