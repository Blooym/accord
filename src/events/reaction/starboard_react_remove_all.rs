use crate::AppState;
use ::serenity::all::{ChannelId, MessageId};
use anyhow::{Error, Result};
use poise::serenity_prelude as serenity;
use sqlx::query;

pub async fn starboard_process_react_remove_all(
    ctx: &serenity::Context,
    _framework: poise::FrameworkContext<'_, AppState, Error>,
    data: &AppState,
    _channel_id: &ChannelId,
    removed_from_message_id: &MessageId,
) -> Result<()> {
    let message_id: i64 = removed_from_message_id.get().try_into()?;
    let starboard_entries_for_message = query!(
        "SELECT starboard_channel_id, starboard_message_id FROM starred_messages WHERE original_message_id = ?1",
        message_id
    )
    .fetch_all(data.database.pool())
    .await?;

    for message_starboard_entry in starboard_entries_for_message {
        ctx.http
            .delete_message(
                ChannelId::new(message_starboard_entry.starboard_channel_id.try_into()?),
                MessageId::new(message_starboard_entry.starboard_message_id.try_into()?),
                None,
            )
            .await?;
        query!(
            "DELETE FROM starred_messages WHERE starboard_message_id = ?1",
            message_starboard_entry.starboard_message_id
        )
        .execute(data.database.pool())
        .await?;
    }

    Ok(())
}
