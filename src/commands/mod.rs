mod starboard_settings;

pub use self::starboard_settings::starboard_settings_sub;
use crate::database::DatabasePool;
use anyhow::Result;
use sqlx::query;

async fn starboard_setup_in_channel(channel_id: i64, pool: &DatabasePool) -> Result<bool> {
    Ok(query!(
        "SELECT channel_id FROM starboards WHERE channel_id = ?1",
        channel_id,
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}
