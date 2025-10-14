mod allow_selfstar;
mod create;
mod delete;
mod emoji;
mod enable;
mod threshold;

use self::{
    allow_selfstar::allow_selfstar, create::create_cmd, delete::delete_cmd, emoji::emoji_cmd,
    enable::enable_cmd, threshold::threshold_cmd,
};
use crate::PoiseContext;
use anyhow::Result;

/// A collection of commands for starboard configuration.
#[poise::command(
    rename = "starboard-settings",
    prefix_command,
    slash_command,
    hide_in_help,
    default_member_permissions = "MANAGE_CHANNELS",
    required_bot_permissions = "VIEW_CHANNEL | SEND_MESSAGES | READ_MESSAGE_HISTORY |  EMBED_LINKS",
    guild_cooldown = 5s,
    subcommand_required,
    subcommands(
        "create_cmd",
        "delete_cmd",
        "enable_cmd",
        "threshold_cmd",
        "emoji_cmd",
        "allow_selfstar"
    )
)]
pub async fn starboard_settings_sub(_: PoiseContext<'_>) -> Result<()> {
    Ok(())
}
