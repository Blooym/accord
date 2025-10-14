use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::Channel;
use sqlx::query;

/// Change the 'star' emoji for a starboard.
#[poise::command(rename = "emoji", prefix_command, slash_command, guild_only)]
pub async fn emoji_cmd(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The starboard to configure"]
    starboard: Channel,
    #[description = "The new emoji to use as the 'star'"] emoji: String,
) -> Result<(), Error> {
    let channel_id = starboard.id().get().try_into()?;
    if !starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard does not exist for that channel.")
            .await?;
        return Ok(());
    }

    if emojis::get(&emoji).is_none() {
        ctx.say("Invalid or unknown emoji. You can only use Discord's default emojis for the starboard.")
            .await?;
        return Ok(());
    };

    query!(
        "UPDATE starboards SET emoji = ?1 WHERE channel_id = ?2",
        emoji,
        channel_id,
    )
    .execute(ctx.data().database.pool())
    .await?;
    ctx.say(format!("Updated starboard emoji to {}.", emoji))
        .await?;

    Ok(())
}
