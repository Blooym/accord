use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::{Channel, Mentionable};
use sqlx::query;

/// Enable or disable a starboard.
#[poise::command(rename = "enable", prefix_command, slash_command, guild_only)]
pub async fn enable_cmd(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The starboard to configure"]
    starboard: Channel,
    #[description = "Whether or not to enable the starboard"] enabled: bool,
) -> Result<(), Error> {
    let channel_id = starboard.id().get().try_into()?;
    if !starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard does not exist for that channel.")
            .await?;
        return Ok(());
    }

    query!(
        "UPDATE starboards SET enabled = ?1 WHERE channel_id = ?2",
        enabled,
        channel_id,
    )
    .execute(ctx.data().database.pool())
    .await?;

    match enabled {
        true => {
            ctx.say(format!(
                "The starboard in {} has been enabled.",
                starboard.mention()
            ))
            .await?;
        }
        false => {
            ctx.say(format!(
                "The starboard in {} has been disabled - no new entries will be made.",
                starboard.mention()
            ))
            .await?;
        }
    }
    Ok(())
}
