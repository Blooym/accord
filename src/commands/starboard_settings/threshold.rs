use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::Channel;
use sqlx::query;

/// Change the trigger threshold for a starboard.
#[poise::command(rename = "threshold", prefix_command, slash_command, guild_only)]
pub async fn threshold_cmd(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The starboard to configure"]
    starboard: Channel,
    #[description = "The amount of reactions needed to post in the starboard"] threshold: u32,
) -> Result<(), Error> {
    let channel_id = starboard.id().get().try_into()?;
    if !starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard does not exist for that channel.")
            .await?;
        return Ok(());
    }

    query!(
        "UPDATE starboards SET threshold = ?1 WHERE channel_id = ?2",
        threshold,
        channel_id,
    )
    .execute(ctx.data().database.pool())
    .await?;

    ctx.say(format!(
        "Set the amount of reactions needed to post in the starboard to **{}**.",
        threshold
    ))
    .await?;

    Ok(())
}
