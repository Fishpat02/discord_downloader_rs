#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

use crate::Error;
use serenity::{Channel, ChannelId, Http, Message, MessagePagination};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub struct MessageLogger {
    http: Arc<Http>,
    ct: Option<CancellationToken>,
}

impl MessageLogger {
    pub fn new(http: Arc<Http>, ct: Option<CancellationToken>) -> Self {
        MessageLogger { http, ct }
    }

    pub async fn get_channels(&self, ids: Vec<ChannelId>) -> Result<Vec<Channel>, Error> {
        let mut channels = vec![];

        for id in ids {
            if let Some(token) = &self.ct
                && token.is_cancelled()
            {
                break;
            }

            channels.push(self.http.get_channel(id).await?);
        }

        Ok(channels)
    }

    pub async fn get_messages_in_channel(&self, channel: Channel) -> Result<Vec<Message>, Error> {
        let mut messages = vec![];
        let mut last_message: Option<&Message> = None;

        loop {
            if let Some(token) = &self.ct
                && token.is_cancelled()
            {
                break;
            }

            if last_message.is_some() {
                let target = MessagePagination::Before(last_message.unwrap().id);
                let mut batch = self
                    .http
                    .get_messages(channel.id(), Some(target), Some(100))
                    .await?;
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
                let mut batch = self
                    .http
                    .get_messages(channel.id(), None, Some(100))
                    .await?;

                if !batch.is_empty() {
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
}
