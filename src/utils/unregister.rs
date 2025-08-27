use crate::commands::*;
use serenity::all as serenity;

#[allow(dead_code)]
async fn unregister() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let token = dotenvy::var("TOKEN")?;
    let intents = serenity::GatewayIntents::all();

    let app_id = serenity::ApplicationId::new(dotenvy::var("CLIENT")?.parse::<u64>()?);
    let http = serenity::Http::new(&token);
    http.set_application_id(app_id);

    let client = serenity::ClientBuilder::new_with_http(http, intents).await?;

    let commands = client.http.get_global_commands().await?;
    for cmd in commands {
        client.http.delete_global_command(cmd.id).await?;
    }

    Ok(())
}
