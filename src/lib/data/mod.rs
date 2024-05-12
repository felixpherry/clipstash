pub mod model;
pub mod query;

use crate::domain::clip::ClipError;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(
                    "If the database hasn't yet been created, run: \n $ sqlx database setup\n"
                );
                panic!("database connection error")
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Debug, Clone, From, Serialize, Deserialize, Display)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
        // match Uuid::from_str(s) {
        //     Ok(uuid) => Ok(uuid.into()),
        //     Err(e) => Err(e.into()),
        // }
    }
}
