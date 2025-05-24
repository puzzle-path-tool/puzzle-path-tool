#![cfg(any(feature = "rusqlite", feature = "tokio-rusqlite"))]

use rusqlite::OptionalExtension;
use url::Url;

#[cfg(feature = "tokio-rusqlite")]
mod rusqlite_async {
    use async_trait::async_trait;

    use crate::url::url_fetcher::{
        UrlFetcher,
        cache::{CacheError, UrlFetcherCache},
    };

    use super::FetchReturn;

    #[derive(Debug, thiserror::Error)]
    pub enum FetchError {
        #[error("Error executing SQL: {0}")]
        SQLError(#[from] tokio_rusqlite::Error),
        #[error("Other parsing Url: {0}")]
        InvalidUrlError(#[from] url::ParseError),
    }

    pub struct RusqliteUrlFetcherCache {
        connection: tokio_rusqlite::Connection,
    }

    impl RusqliteUrlFetcherCache {
        #[must_use]
        pub fn new(connection: tokio_rusqlite::Connection) -> Self {
            RusqliteUrlFetcherCache { connection }
        }
    }

    #[async_trait]
    impl UrlFetcherCache for RusqliteUrlFetcherCache {
        type FetchError = FetchError;
        type StoreError = tokio_rusqlite::Error;

        async fn store_redirect(
            &self,
            url: url::Url,
            value: Option<url::Url>,
        ) -> Result<(), Self::StoreError> {
            self.connection
                .call(move |connection| {
                    super::store_redirect(connection, &url, value.as_ref())?;
                    Ok(())
                })
                .await
        }

        async fn store_result(
            &self,
            url: url::Url,
            value: Box<str>,
        ) -> Result<(), Self::StoreError> {
            self.connection
                .call(move |connection| {
                    super::store_result(connection, &url, value.as_ref())?;
                    Ok(())
                })
                .await
        }
    }

    #[async_trait]
    impl UrlFetcher for RusqliteUrlFetcherCache {
        type Error = CacheError<FetchError>;

        async fn fetch_redirect_url(
            &self,
            _url: url::Url,
        ) -> Result<Option<url::Url>, Self::Error> {
            todo!("THIS IS TYPE HELL")

            // let result = self
            //     .connection
            //     .call(
            //         move |connection| match super::fetch_redirect_url(&connection, &url) {
            //             Err(FetchRedirectError::SQLError(err)) => Err(err.into()),
            //             v => Ok(v),
            //         },
            //     )
            //     .await;

            // let result = match result {
            //     Ok(value) => value,
            //     Err(err) => {
            //         return Err(CacheError::CacheFetchError(Box::new(FetchError::SQLError(
            //             err,
            //         ))));
            //     }
            // };

            // match result {
            //     Ok(FetchReturn::Found(value)) => Ok(value),
            //     Ok(FetchReturn::NotThere) => Err(CacheError::NoCacheValue),
            //     Err(FetchRedirectError::InvalidUrlError(err)) => Err(CacheError::CacheFetchError(Box::new(FetchError::SQLError(
            //         err,
            //     )))),
            //     Err(err) => Err(CacheError::CacheFetchError(Box::new(FetchError::SQLError(
            //         err,
            //     )))),
            // }
        }

        async fn fetch_result(&self, url: url::Url) -> Result<Box<str>, Self::Error> {
            let result = self
                .connection
                .call(move |connection| {
                    let fetch_return = super::fetch_result(connection, &url)?;
                    Ok(fetch_return)
                })
                .await;

            match result {
                Ok(FetchReturn::Found(value)) => Ok(value),
                Ok(FetchReturn::NotThere) => Err(CacheError::NoCacheValue),
                Err(err) => Err(CacheError::CacheFetchError(Box::new(FetchError::SQLError(
                    err,
                )))),
            }
        }
    }
}
#[cfg(feature = "tokio-rusqlite")]
pub use rusqlite_async::{FetchError, RusqliteUrlFetcherCache};

#[cfg(feature = "rusqlite")]
mod rusqlite_blocking {
    use crate::url::url_fetcher::{
        BlockingUrlFetcher,
        cache::{BlockingUrlFetcherCache, CacheError},
    };

    use super::{FetchRedirectError, FetchReturn};

    #[derive(Debug, thiserror::Error)]
    pub enum BlockingFetchError {
        #[error("Error executing SQL: {0}")]
        SQLError(#[from] rusqlite::Error),
        #[error("Other parsing Url: {0}")]
        InvalidUrlError(#[from] url::ParseError),
    }

    impl BlockingFetchError {
        fn from_redirect_error(err: FetchRedirectError) -> Self {
            match err {
                FetchRedirectError::InvalidUrlError(url) => Self::InvalidUrlError(url),
                FetchRedirectError::SQLError(err) => Self::SQLError(err),
            }
        }
    }

    pub struct RusqliteBlockingUrlFetcherCache {
        connection: rusqlite::Connection,
    }

    impl RusqliteBlockingUrlFetcherCache {
        pub fn new(connection: rusqlite::Connection) -> Self {
            RusqliteBlockingUrlFetcherCache { connection }
        }
    }

    impl BlockingUrlFetcherCache for RusqliteBlockingUrlFetcherCache {
        type FetchError = BlockingFetchError;
        type StoreError = rusqlite::Error;

        fn store_redirect_blocking(
            &self,
            url: url::Url,
            value: Option<url::Url>,
        ) -> Result<(), Self::StoreError> {
            super::store_redirect(&self.connection, &url, value.as_ref())
        }

        fn store_result_blocking(
            &self,
            url: url::Url,
            value: Box<str>,
        ) -> Result<(), Self::StoreError> {
            super::store_result(&self.connection, &url, value.as_ref())
        }
    }

    impl BlockingUrlFetcher for RusqliteBlockingUrlFetcherCache {
        type Error = CacheError<BlockingFetchError>;

        fn fetch_redirect_url_blocking(
            &self,
            url: url::Url,
        ) -> Result<Option<url::Url>, Self::Error> {
            match super::fetch_redirect_url(&self.connection, &url) {
                Ok(FetchReturn::Found(value)) => Ok(value),
                Ok(FetchReturn::NotThere) => Err(CacheError::NoCacheValue),
                Err(err) => Err(CacheError::CacheFetchError(Box::new(
                    BlockingFetchError::from_redirect_error(err),
                ))),
            }
        }

        fn fetch_result_blocking(&self, url: url::Url) -> Result<Box<str>, Self::Error> {
            match super::fetch_result(&self.connection, &url) {
                Ok(FetchReturn::Found(value)) => Ok(value),
                Ok(FetchReturn::NotThere) => Err(CacheError::NoCacheValue),
                Err(err) => Err(CacheError::CacheFetchError(Box::new(
                    BlockingFetchError::SQLError(err),
                ))),
            }
        }
    }
}

#[cfg(feature = "rusqlite")]
pub use rusqlite_blocking::{BlockingFetchError, RusqliteBlockingUrlFetcherCache};

fn store_redirect(
    connection: &rusqlite::Connection,
    url: &url::Url,
    value: Option<&url::Url>,
) -> rusqlite::Result<()> {
    connection.execute(
        "INSERT OR REPLACE INTO redirect_cache (request_url, redirect_url) VALUES (?1, ?2)",
        (url.to_string(), value.map(url::Url::to_string)),
    )?;
    Ok(())
}

fn store_result(
    connection: &rusqlite::Connection,
    url: &url::Url,
    value: &str,
) -> rusqlite::Result<()> {
    connection.execute(
        "INSERT OR REPLACE INTO result_cache (request_url, response_value) VALUES (?1, ?2)",
        (url.to_string(), value.to_string()),
    )?;
    Ok(())
}

#[derive(Debug, Clone)]
enum FetchReturn<T> {
    Found(T),
    NotThere,
}

impl<T> FetchReturn<T> {
    fn from_option(option: Option<T>) -> Self {
        match option {
            Some(v) => Self::Found(v),
            None => Self::NotThere,
        }
    }

    fn into_option(self) -> Option<T> {
        match self {
            Self::Found(v) => Some(v),
            Self::NotThere => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum FetchRedirectError {
    #[error("Error executing SQL: {0}")]
    SQLError(#[from] rusqlite::Error),
    #[error("Other parsing Url: {0}")]
    InvalidUrlError(#[from] url::ParseError),
}

fn fetch_redirect_url(
    connection: &rusqlite::Connection,
    url: &url::Url,
) -> Result<FetchReturn<Option<url::Url>>, FetchRedirectError> {
    let mut statement =
        connection.prepare("SELECT redirect_url FROM redirect_cache WHERE request_url=?1")?;

    let redirect_url: Option<Option<String>> = statement
        .query_row((url.to_string(),), |row| row.get(0))
        .optional()?;

    let fetch_return = match redirect_url {
        Some(Some(url_string)) => FetchReturn::Found(Some(Url::parse(&url_string)?)),
        Some(None) => FetchReturn::Found(None),
        None => FetchReturn::NotThere,
    };

    Ok(fetch_return)
}

fn fetch_result(
    connection: &rusqlite::Connection,
    url: &url::Url,
) -> rusqlite::Result<FetchReturn<Box<str>>> {
    let mut statement =
        connection.prepare("SELECT response_value FROM result_cache WHERE request_url=?1")?;

    let redirect_url: Option<String> = statement
        .query_row((url.to_string(),), |row| row.get(0))
        .optional()?;

    let fetch_return =
        FetchReturn::from_option(redirect_url.map(std::string::String::into_boxed_str));

    Ok(fetch_return)
}
