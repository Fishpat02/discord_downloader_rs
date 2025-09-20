#[cfg(feature = "bot")]
use serenity::all as serenity;

#[cfg(feature = "self-bot")]
use serenity_self::all as serenity;

use crate::Error;
use serenity::{Channel, ChannelId, Http, Message, MessagePagination};
use std::sync::Arc;

pub async fn get_channels(http: Arc<Http>, ids: Vec<ChannelId>) -> Result<Vec<Channel>, Error> {
    let mut channels = vec![];

    for id in ids {
        channels.push(http.get_channel(id).await?);
    }

    Ok(channels)
}

