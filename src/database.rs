use anyhow::Result;
use sqlx::{
    SqlitePool, migrate,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode},
};
use std::str::FromStr;

pub type DatabasePool = SqlitePool;

pub struct Database {
    pool: DatabasePool,
}

impl Database {
    pub async fn new(connect_string: &str) -> Result<Self> {
        let pool = DatabasePool::connect_with(
            SqliteConnectOptions::from_str(connect_string)?
                .journal_mode(SqliteJournalMode::Wal)
                .auto_vacuum(SqliteAutoVacuum::Full)
                .optimize_on_close(true, None)
                .foreign_keys(true)
                .create_if_missing(true),
        )
        .await?;
        migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }
}
