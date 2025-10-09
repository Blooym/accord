mod commands;
mod database;
mod events;

use crate::events::event_handler;
use crate::{commands::starboard_settings_sub, database::Database};
use anyhow::{Context, Error, Result};
use clap::Parser;
use dotenvy::dotenv;
use poise::serenity_prelude::{
    ActivityData, ClientBuilder, CreateAllowedMentions, GatewayIntents, OnlineStatus,
};
use tracing_subscriber::EnvFilter;

type PoiseContext<'a> = poise::Context<'a, AppState, Error>;

struct AppState {
    database: Database,
}

#[derive(Debug, Parser)]
#[clap(about, author, version)]
struct AppSettings {
    /// The local database URL to use for persistent storage.
    #[clap(long = "database-url", env = "DATABASE_URL")]
    database_url: String,

    /// The Discord bot token to authenticate with.
    #[clap(long = "discord-token", env = "ACCORD_DISCORD_TOKEN")]
    discord_token: String,

    /// The guild to register commands for testing.
    #[cfg(debug_assertions)]
    #[clap(long = "discord-dev-guild-id", env = "ACCORD_DEV_GUILD_ID")]
    discord_dev_guild_id: Option<u64>,

    /// The custom status to use for the bot's profile
    #[clap(long = "discord-bot-status", env = "ACCORD_DISCORD_BOT_STATUS")]
    discord_bot_status: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init();
    let args = AppSettings::parse();
    let database = Database::new(&args.database_url)
        .await
        .context("failed to initialise database")?;

    let framework = poise::Framework::<AppState, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![starboard_settings_sub()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            allowed_mentions: Some(
                CreateAllowedMentions::default()
                    .everyone(false)
                    .all_users(false)
                    .all_roles(false)
                    .replied_user(true),
            ),
            initialize_owners: true,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                #[cfg(debug_assertions)]
                {
                    if let Some(dev_guild_id) = args.discord_dev_guild_id {
                        use serenity::all::GuildId;
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            GuildId::from(dev_guild_id),
                        )
                        .await?;
                    }
                }
                #[cfg(not(debug_assertions))]
                {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                if let Some(discord_bot_status) = args.discord_bot_status {
                    ctx.set_presence(
                        Some(ActivityData::custom(discord_bot_status)),
                        OnlineStatus::DoNotDisturb,
                    );
                }

                Ok(AppState { database })
            })
        })
        .build();

    // Start the bot.
    ClientBuilder::new(
        args.discord_token,
        GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MESSAGE_REACTIONS
            | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await?
    .start_autosharded()
    .await
    .context("Error while starting client")
}
