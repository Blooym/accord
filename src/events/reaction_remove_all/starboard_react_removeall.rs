use crate::AppState;
use ::serenity::all::{ChannelId, MessageId, ReactionType};
use anyhow::{Error, Result};
use poise::serenity_prelude as serenity;
use serenity::all::Reaction;
use sqlx::query;
use tracing::{error, warn};

pub async fn starboard_react_removeall(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, AppState, Error>,
    data: &AppState,
    reaction: &Reaction,
) -> Result<()> {
    // Events that do not occur in guilds are ignored.
    let guild_id: i64 = match reaction.guild_id.map(|g| g.get().try_into().unwrap()) {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };
    // Since the bot does not work with custom/super emojis, we can skip events that contain them.
    let emoji = match &reaction.emoji {
        ReactionType::Unicode(emoji) => emoji,
        _ => return Ok(()),
    };
    if reaction.burst {
        return Ok(());
    }

    // Fetch the message that was reacted to or return if it doesn't exist anymore.
    let message = match reaction.message(&ctx.http).await {
        Ok(message) => message,
        Err(e) => {
            error!("Failed to get message: {:?}", e);
            return Ok(());
        }
    };
    let message_id: i64 = message.id.get().try_into()?;

    // Find applicable starboards for the event. Skip if none.
    let starboards = query!(
        "SELECT channel_id, enabled, emoji, allow_selfstar, threshold FROM starboards WHERE guild_id = ?1 AND emoji = ?2",
        guild_id,
        emoji
    )
    .fetch_all(data.database.pool())
    .await?;
    if starboards.is_empty() {
        return Ok(());
    }

    for starboard in starboards {
        // We don't need to do anything if
        // - The starboard is disabled.
        // - The message was sent in the starboard channel.
        // - The reaction is not the starboard emoji.
        if !starboard.enabled
            || i64::try_from(reaction.channel_id.get())? == starboard.channel_id
            || reaction.emoji != ReactionType::Unicode(starboard.emoji.clone())
        {
            return Ok(());
        }

        let starboard_channel = ChannelId::new(starboard.channel_id.try_into()?);

        // Find existing message.
        match query!(
            "SELECT starboard_message_id FROM starred_messages WHERE starboard_id = ?1 AND message_id = ?2",
            starboard.channel_id,
            message_id
        )
        .fetch_optional(data.database.pool())
        .await?
        {
            Some(starboard_message) => {
                match starboard_channel
                    .message(
                        &ctx.http,
                        MessageId::from(u64::try_from(starboard_message.starboard_message_id)?),
                    )
                    .await
                {
                    Ok(message) => {
                        match message.delete(&ctx.http).await {
                            Ok(_) => (),
                            Err(err) => {
                                warn!("Failed to delete starboard message: {err:?}");
                            }
                        };
                    }
                    Err(err) => {
                        warn!(
                            "Error while finding message in starboard channel for starboard entry, skipping message deletion: {err:?}"
                        );
                    }
                };
            }
            _ => {
                warn!("No message found in database for starboard entry, cannot delete message");
                return Ok(());
            }
        };

        query!(
            "DELETE FROM starred_messages WHERE starboard_id = ?1 AND message_id = ?2",
            starboard.channel_id,
            message_id
        )
        .execute(data.database.pool())
        .await?;
    }

    Ok(())
}
