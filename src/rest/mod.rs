#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

use crate::Error;
use serenity::{Channel, ChannelId, Http, Message, MessagePagination};
use std::sync::Arc;

#[allow(dead_code)]
pub async fn get_channels(http: Arc<Http>, ids: Vec<ChannelId>) -> Result<Vec<Channel>, Error> {
    let mut channels = vec![];

    for id in ids {
        channels.push(http.get_channel(id).await?);
    }

    Ok(channels)
}

#[allow(dead_code)]
pub async fn get_messages_in_channel(
    http: Arc<Http>,
    channel: Channel,
) -> Result<Vec<Message>, Error> {
    let mut messages = vec![];
    let mut last_message: Option<&Message> = None;

    loop {
        if let Some(_) = last_message {
            let target = MessagePagination::Before(last_message.unwrap().id);
            let mut batch = http.get_messages(channel.id(), Some(target), Some(100)).await?;
            let batch_len = batch.len();

            if batch_len > 0 {
                messages.append(&mut batch);
                last_message = messages.last();

                println!(
                    "Retrieved {} messages in channel {} from {}",
                    batch_len,
                    channel,
                    last_message.unwrap().timestamp
                );
            } else {
                break;
            }
        } else {
            let mut batch = http.get_messages(channel.id(), None, Some(50)).await?;

            if batch.len() > 0 {
                messages.append(&mut batch);
                last_message = messages.last();

                println!(
                    "Retrieved {} messages in channel {} from {}",
                    messages.len(),
                    channel,
                    last_message.unwrap().timestamp
                );
            } else {
                break;
            }
        }
    }

    Ok(messages)
}
