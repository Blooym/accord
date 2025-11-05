use crate::{Error, PoiseContext, commands::starboard_setup_in_channel};
use poise::serenity_prelude::Channel;
use sqlx::query;

/// Create a new starboard.
#[poise::command(rename = "create", prefix_command, slash_command, guild_only)]
pub async fn create_cmd(
    ctx: PoiseContext<'_>,
    #[channel_types("Text")]
    #[description = "The channel to create the starboard in"]
    channel: Channel,
    #[description = "The amount of reactions needed to post to the starboard"]
    #[min = 1]
    threshold: u32,
    #[description = "The emoji to use for as the 'star'"] emoji: String,
    #[description = "Count users reacting to their own messages"] allow_selfstar: Option<bool>,
) -> Result<(), Error> {
    let Some(guild_id): Option<i64> = ctx.guild_id().map(|g| g.get().try_into().unwrap()) else {
        ctx.say("This command can only be used in a guild.").await?;
        return Ok(());
    };

    // Ensure the given emoji is valid.
    if emojis::get(&emoji).is_none() {
        ctx.say("Invalid or unknown emoji. You can only use Discord's default emojis for the starboard.")
            .await?;
        return Ok(());
    };

    // Ensure that the channel is not already a starboard.
    let channel_id = channel.id().get().try_into()?;
    if starboard_setup_in_channel(channel_id, ctx.data().database.pool()).await? {
        ctx.say("A starboard is already configured for that channel! Either remove the starboard or use another channel instead.").await?;
        return Ok(());
    }

    // Create starboard.
    let allow_selfstar = allow_selfstar.unwrap_or(false);
    query!("INSERT OR IGNORE INTO guilds (id) VALUES (?1)", guild_id)
        .execute(ctx.data().database.pool())
        .await?;
    query!(
        "INSERT INTO starboards (guild_id, enabled, channel_id, emoji, threshold, allow_selfstar) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        guild_id,
        true,
        channel_id,
        emoji,
        threshold,
        allow_selfstar
    )
    .execute(ctx.data().database.pool())
    .await?;

    ctx.say(format!(
        "Successfully created starboard for <#{}>.",
        channel_id
    ))
    .await?;

    Ok(())
}
