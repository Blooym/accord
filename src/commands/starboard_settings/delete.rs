use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::Channel;
use sqlx::query;

/// DANGER: Delete a starboard. Breaks all ties between a starboard message & original message.
#[poise::command(rename = "delete", prefix_command, slash_command, guild_only)]
pub async fn delete_cmd(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The starboard to delete"]
    starboard: Channel,
) -> Result<(), Error> {
    let channel_id = starboard.id().get().try_into()?;
    if !starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard does not exist for that channel.")
            .await?;
        return Ok(());
    }

    // TODO: interaction with warning about data loss.

    query!("DELETE FROM starboards WHERE channel_id = ?1", channel_id)
        .execute(ctx.data().database.pool())
        .await?;
    ctx.say("The starboard in that channel has been deleted successfully and all recorded messages have been removed from storage.").await?;

    Ok(())
}
