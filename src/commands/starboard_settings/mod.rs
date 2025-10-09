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
    default_member_permissions = "MANAGE_GUILD | MANAGE_CHANNELS",
    required_bot_permissions = "MANAGE_CHANNELS | VIEW_CHANNEL | READ_MESSAGE_HISTORY | SEND_MESSAGES | EMBED_LINKS",
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
