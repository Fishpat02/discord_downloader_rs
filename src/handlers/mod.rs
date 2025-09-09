#[cfg(feature = "bot")]
use serenity::all::*;

#[cfg(feature = "self-bot")]
use serenity_self::all::*;

pub struct CustomHandler;

#[async_trait]
impl EventHandler for CustomHandler {
    async fn message(&self, _ctx: Context, new_message: Message) {
        let user = _ctx.http.get_current_user().await.unwrap();

        if new_message.author == user.into() {
            println!("Said: {}", new_message.content);
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!(
            "Logged in as {}#{}!",
            data_about_bot.user.name,
            data_about_bot.user.discriminator.unwrap()
        );
    }
}
