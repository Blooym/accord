mod reaction;

use crate::{
    AppState,
    events::reaction::{
        starboard_process_react_add, starboard_process_react_remove,
        starboard_process_react_remove_all,
    },
};
use anyhow::{Error, Result};
use poise::serenity_prelude::{Context, FullEvent};
use tracing::info;

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, AppState, Error>,
    data: &AppState,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::ReactionAdd { add_reaction } => {
            starboard_process_react_add(ctx, framework, data, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            starboard_process_react_remove(ctx, framework, data, removed_reaction).await?;
        }
        FullEvent::ReactionRemoveAll {
            channel_id,
            removed_from_message_id,
        } => {
            starboard_process_react_remove_all(
                ctx,
                framework,
                data,
                channel_id,
                removed_from_message_id,
            )
            .await?;
        }
        _ => {}
    }
    Ok(())
}
