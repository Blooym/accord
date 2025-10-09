use crate::{
    AppState,
    events::{count_reactors, make_starboard_message},
};
use ::serenity::all::{ChannelId, CreateMessage, EditMessage, MessageFlags, MessageId};
use anyhow::{Error, Result, bail};
use poise::serenity_prelude as serenity;
use serenity::all::{Reaction, ReactionType};
use sqlx::query;
use tracing::{error, warn};

pub async fn starboard_react_remove(
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

    // Fetch the message that was reacted to..
    let message = match reaction.message(&ctx.http).await {
        Ok(message) => message,
        Err(e) => {
            error!("Failed to get message from react event: {:?}", e);
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
        // Ignore people reacting to their own message unless it's allowed.
        if !starboard.allow_selfstar {
            match reaction.user_id {
                Some(reactor_id) => {
                    if reactor_id == message.author.id {
                        return Ok(());
                    }
                }
                None => {
                    bail!(
                        "Message reactor UserID not available while handling a starboard reaction removal event."
                    )
                }
            };
        }

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

        // Get a list of users that reacted to the message.
        let reactors_count: i64 = count_reactors(&message, &ctx.http, &reaction.emoji, |r| {
            (starboard.allow_selfstar || r.id != message.author.id) && !r.bot
        })
        .await?
        .try_into()?;

        // Try find existing starboard message.
        let msg_parts = make_starboard_message(
            &message,
            starboard.emoji,
            reactors_count.try_into()?,
            starboard.threshold.try_into()?,
        );
        let starboard_channel = ChannelId::new(starboard.channel_id.try_into()?);
        let starboard_message = match query!(
            "SELECT starboard_message_id FROM starred_messages WHERE starboard_id = ?1 AND message_id = ?2",
            starboard.channel_id,
            message_id
        )
        .fetch_optional(data.database.pool())
        .await?
        { Some(starboard_message) => {
            match starboard_channel
                .message(
                    &ctx.http,
                    MessageId::from(u64::try_from(starboard_message.starboard_message_id)?),
                )
                .await
            {
                Ok(mut message) => {
                    // If under threshhold, delete existing message.
                    if reactors_count < starboard.threshold {
                        message.delete(&ctx.http).await?;
                        query!("DELETE FROM starred_messages WHERE starboard_message_id = ?1", starboard_message.starboard_message_id)
                            .execute(data.database.pool()).await?;
                        return Ok(());
                    }

                    // Otherwise update it with the new reactors_count.
                    message
                        .edit(&ctx.http, EditMessage::new().content(msg_parts.content).embed(msg_parts.embed).flags(MessageFlags::SUPPRESS_NOTIFICATIONS))
                        .await?;
                    message
                }
                Err(err) => {
                    // If under threshhold, delete the broken record instead of recreating the message.
                    if reactors_count < starboard.threshold {
                        query!("DELETE FROM starred_messages WHERE starboard_message_id = ?1", starboard_message.starboard_message_id)
                            .execute(data.database.pool()).await?;
                        return Ok(());
                    }

                    // Otherwise recreate the message.
                    warn!("Caught error when fetching existing starboard message, making new message: {err:?}");
                    starboard_channel
                        .send_message(&ctx.http, CreateMessage::new().content(msg_parts.content).embed(msg_parts.embed).flags(MessageFlags::SUPPRESS_NOTIFICATIONS))
                        .await?
                }
            }
        } _ => {
            // Check if the message should be posted.
            if reactors_count < starboard.threshold {
                return Ok(());
            }

            // Send the message to the starboard.
            starboard_channel
                .send_message(
                    &ctx.http,
                    CreateMessage::new().content(msg_parts.content).embed(msg_parts.embed).flags(MessageFlags::SUPPRESS_NOTIFICATIONS),
                )
                .await?
        }};

        // Add/update the entry in the database.
        let author_id: i64 = message.author.id.get().try_into()?;
        let starboard_message_id: i64 = starboard_message.id.get().try_into()?;
        query!(
            "INSERT INTO starred_messages
                (starboard_message_id, starboard_id, message_id, author_id, react_count) VALUES
                (?1, ?2, ?3, ?4, ?5) 
                ON CONFLICT (starboard_id, message_id) DO UPDATE
                SET starboard_message_id = ?1, react_count = ?5",
            starboard_message_id,
            starboard.channel_id,
            message_id,
            author_id,
            reactors_count
        )
        .execute(data.database.pool())
        .await?;
    }

    Ok(())
}
