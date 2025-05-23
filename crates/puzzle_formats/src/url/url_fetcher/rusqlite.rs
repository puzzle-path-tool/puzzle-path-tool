#![cfg(feature = "rusqlite")]

use rusqlite::Connection;

use super::{
    BlockingUrlFetcher,
    cache::{BlockingUrlFetcherCache, CacheError},
};

pub struct RusqliteUrlFetcherCache {
    connection: Connection,
}

impl RusqliteUrlFetcherCache {
    pub fn new(connection: Connection) -> Self {
        RusqliteUrlFetcherCache { connection }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Placeholder Error")]
    Placeholder,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Placeholder Error")]
    Placeholder,
}

impl BlockingUrlFetcherCache for RusqliteUrlFetcherCache {
    type FetchError = FetchError;
    type StoreError = StoreError;

    fn store_redirect_blocking(
        &self,
        url: url::Url,
        value: Option<url::Url>,
    ) -> Result<(), Self::StoreError> {
        todo!()
    }

    fn store_result_blocking(
        &self,
        url: url::Url,
        value: Box<str>,
    ) -> Result<(), Self::StoreError> {
        todo!()
    }
}

impl BlockingUrlFetcher for RusqliteUrlFetcherCache {
    type Error = CacheError<FetchError>;

    fn fetch_redirect_url_blocking(&self, url: url::Url) -> Result<Option<url::Url>, Self::Error> {
        todo!()
    }

    fn fetch_result_blocking(&self, url: url::Url) -> Result<Box<str>, Self::Error> {
        todo!()
    }
}
