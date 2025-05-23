#![cfg(feature = "tokio_rusqlite")]

use async_trait::async_trait;
use rusqlite::OptionalExtension;
use tokio_rusqlite::Connection;
use url::Url;

use super::{
    UrlFetcher,
    cache::{CacheError, UrlFetcherCache},
};

pub struct TokioRusqliteUrlFetcherCache {
    connection: Connection,
}

impl TokioRusqliteUrlFetcherCache {
    #[allow(clippy::missing_errors_doc)]
    pub async fn new(connection: Connection) -> tokio_rusqlite::Result<Self> {
        connection
            .call(|connection| {
                connection.execute(
                    "CREATE TABLE IF NOT EXISTS result_cache (
                    request_url     TEXT NOT NULL PRIMARY KEY,
                    response_value  TEXT NOT NULL
                )",
                    (),
                )?;
                connection.execute(
                    "CREATE TABLE IF NOT EXISTS redirect_cache (
                    request_url     TEXT NOT NULL PRIMARY KEY,
                    redirect_url    TEXT
                )",
                    (),
                )?;

                Ok(())
            })
            .await?;

        Ok(TokioRusqliteUrlFetcherCache { connection })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Error executing SQL: {0}")]
    SQLError(#[from] tokio_rusqlite::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Error executing SQL: {0}")]
    SQLError(#[from] tokio_rusqlite::Error),
}

#[async_trait]
impl UrlFetcherCache for TokioRusqliteUrlFetcherCache {
    type FetchError = FetchError;
    type StoreError = StoreError;

    async fn store_redirect(
        &self,
        url: url::Url,
        value: Option<url::Url>,
    ) -> Result<(), Self::StoreError> {
        self.connection
            .call(move |connection| {
                connection.execute(
                    "INSERT OR REPLACE INTO redirect_cache (request_url, redirect_url) VALUES (?1, ?2)",
                    (url.to_string(), value.map(|u: url::Url| u.to_string())),
                )?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn store_result(&self, url: url::Url, value: Box<str>) -> Result<(), Self::StoreError> {
        self.connection
            .call(move |connection| {
                connection.execute(
                    "INSERT OR REPLACE INTO result_cache (request_url, response_value) VALUES (?1, ?2)",
                    (url.to_string(), value.to_string()),
                )?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}

#[async_trait]
impl UrlFetcher for TokioRusqliteUrlFetcherCache {
    type Error = CacheError<FetchError>;

    async fn fetch_redirect_url(&self, url: url::Url) -> Result<Option<url::Url>, Self::Error> {
        let redirect_url = self
            .connection
            .call(move |connection| {
                let mut statement = connection
                    .prepare("SELECT redirect_url FROM redirect_cache WHERE request_url=?1")?;

                let redirect_url: Result<Option<String>, CacheError<_>> = statement
                    .query_row((url.to_string(),), |row| row.get(0))
                    .optional()?
                    .ok_or(CacheError::NoCacheValue);

                Ok(redirect_url)
            })
            .await?;

        //TODO: Black Magic

        Url::parse(redirect_url)?
    }

    async fn fetch_result(&self, url: url::Url) -> Result<Box<str>, Self::Error> {
        todo!()
    }
}
