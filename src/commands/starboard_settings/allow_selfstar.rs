use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::Channel;
use sqlx::query;

/// Change the selfstar setting for a starboard.
#[poise::command(rename = "allow-selfstar", prefix_command, slash_command, guild_only)]
pub async fn allow_selfstar(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The starboard to configure"]
    starboard: Channel,
    #[description = "Allow users to 'star' their own messages"] allow_selfstar: bool,
) -> Result<(), Error> {
    let channel_id = starboard.id().get().try_into()?;
    if !starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard does not exist for that channel.")
            .await?;
        return Ok(());
    }

    query!(
        "UPDATE starboards SET allow_selfstar = ?1 WHERE channel_id = ?2",
        allow_selfstar,
        channel_id,
    )
    .execute(ctx.data().database.pool())
    .await?;

    ctx.say(format!(
        "Updated starboard setting 'allow selfstar' to **{}**.",
        allow_selfstar
    ))
    .await?;

    Ok(())
}
