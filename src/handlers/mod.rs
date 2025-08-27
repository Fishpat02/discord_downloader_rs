use poise::async_trait;
use serenity::all::*;

pub struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!(
            "Logged in as {}#{}!",
            data_about_bot.user.name,
            data_about_bot.user.discriminator.unwrap()
        );
    }
}
