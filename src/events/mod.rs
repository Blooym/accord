mod reaction_add;
mod reaction_remove;
mod reaction_remove_all;

use self::{
    reaction_add::starboard_react_addr::starboard_react_add,
    reaction_remove::starboard_react_remove::starboard_react_remove,
    reaction_remove_all::starboard_react_removeall::starboard_react_removeall,
};
use crate::AppState;
use anyhow::{Error, Result};
use linkify::LinkFinder;
use poise::serenity_prelude::{Context, FullEvent};
use serenity::all::{
    Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Http, Message, ReactionType, User,
};
use tracing::{info, warn};
use url::Url;

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, AppState, Error>,
    data: &AppState,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::ReactionAdd { add_reaction } => {
            starboard_react_add(ctx, framework, data, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            starboard_react_remove(ctx, framework, data, removed_reaction).await?;
        }
        FullEvent::ReactionRemoveEmoji { removed_reactions } => {
            starboard_react_removeall(ctx, framework, data, removed_reactions).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn count_reactors<F>(
    message: &Message,
    http: impl AsRef<Http>,
    emoji: &ReactionType,
    filter: F,
) -> Result<usize>
where
    F: Fn(&User) -> bool,
{
    let mut reactors_count = 0;
    let mut after = None;
    loop {
        match message
            .reaction_users(&http, emoji.clone(), Some(100), after)
            .await
        {
            Ok(reactors) => {
                let len = reactors.len();
                reactors_count += reactors.iter().filter(|r| filter(r)).count();
                if len < 100 {
                    break;
                }
                after = reactors.last().map(|u| u.id);
            }
            Err(e) => {
                warn!("Unable to get reactors for message {}: {}", message.id, e);
                return Err(e.into());
            }
        }
    }
    Ok(reactors_count)
}

struct StarboardMessageParts {
    pub content: String,
    pub embed: CreateEmbed,
}

/// Get the components needed to create a starboard message.
fn make_starboard_message(
    original_message: &Message,
    emoji: String,
    react_count: usize,
    reacts_needed: usize,
) -> StarboardMessageParts {
    StarboardMessageParts {
        embed: make_starboard_embed(original_message, &emoji, react_count, reacts_needed),
        content: original_message.link(),
    }
}

/// Creates an embed for a starboard message.
fn make_starboard_embed(
    message: &Message,
    emoji: &str,
    react_count: usize,
    reacts_needed: usize,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default()
        .author(CreateEmbedAuthor::new(&message.author.name).icon_url(message.author.face()))
        .footer(CreateEmbedFooter::new(format!(
            "{} {}  • {}",
            emoji, react_count, message.id
        )))
        .timestamp(message.timestamp)
        .colour(match react_count {
            count if count < reacts_needed * 2 => Colour::DARK_ORANGE, // between minimum and 2x
            count if count < reacts_needed * 3 => Colour::ORANGE, // between 2x to 3x of minimum,
            _ => Colour::GOLD,                                    // 3x or higher of minimum
        });

    // Add message content
    if !message.content.is_empty() {
        embed = embed.description(&message.content);
    }

    // Add the first attachment as an image or add a field with all attachments.
    if let Some(attachment) = message.attachments.first() {
        let content_type = attachment.content_type.as_deref().unwrap_or("");
        embed = if content_type.starts_with("image") {
            embed.image(&attachment.url)
        } else {
            embed.field(
                "Attachments",
                message
                    .attachments
                    .iter()
                    .map(|a| format!("[{}]({})", a.filename, a.url))
                    .collect::<Vec<_>>()
                    .join("\n"),
                false,
            )
        };
    }
    // Add the first attachment as an image or add a field with all attachments.
    else if let Some(link) =
        find_all_image_urls_in_str(&message.content).and_then(|links| links.into_iter().next())
    {
        embed = embed.image(link.to_string());
    }
    // Embed the first image link in the message.
    else if let Some(image_url) = message
        .embeds
        .iter()
        .find_map(|e| e.image.as_ref().map(|img| &img.url))
    {
        embed = embed.image(image_url);
    }

    // Add reply context
    if let Some(reply) = &message.referenced_message {
        const MAX_REPLY_LENGTH: usize = 524;

        let reply_content = if reply.content.is_empty() {
            "*sent an attachment, embed or sticker.*".to_string()
        } else if reply.content.len() > MAX_REPLY_LENGTH {
            format!("{}...", &reply.content[..MAX_REPLY_LENGTH])
        } else {
            reply.content.clone()
        };

        embed = embed.field(
            format!("In reply to {}'s message", reply.author.name),
            reply_content,
            false,
        );
    }

    embed
}

/// Returns a vec with all complete image links inside of the given string.
///
/// File types are checked case-insensitively.
///
/// ## Media Support
/// Media support makes a best effort to mirror the formats that Discord can use inside of an image field in an embed.
///
/// * `jpg`
/// * `jpeg`
/// * `png`
/// * `gif`
fn find_all_image_urls_in_str(s: &str) -> Option<Vec<Url>> {
    if s.is_empty() {
        return None;
    }

    let links: Vec<_> = LinkFinder::new().links(s).collect();
    if links.is_empty() {
        return None;
    }

    Some(
        links
            .iter()
            .filter_map(|link| {
                let link_str = link.as_str().to_lowercase();
                if link_str.is_empty() {
                    return None;
                }

                if !link_str.contains(".jpg")
                    && !link_str.contains(".jpeg")
                    && !link_str.contains(".png")
                    && !link_str.contains(".gif")
                {
                    return None;
                }
                Url::parse(link.as_str()).ok()
            })
            .collect(),
    )
}
