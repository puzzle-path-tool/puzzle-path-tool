#![cfg(feature = "rusqlite")]

use crate::url::url_fetcher::{
    BlockingUrlFetcher,
    cache::{BlockingUrlFetcherCache, CacheError},
};

use super::FetchReturn;

#[derive(Debug, thiserror::Error)]
pub enum BlockingFetchError {
    #[error("Error executing SQL: {0}")]
    SQLError(#[from] rusqlite::Error),
    #[error("Other parsing Url: {0}")]
    InvalidUrlError(#[from] url::ParseError),
}

pub struct RusqliteBlockingUrlFetcherCache {
    connection: rusqlite::Connection,
}

impl RusqliteBlockingUrlFetcherCache {
    /// Create a new Instance.
    /// This will also create database tables via the connection.
    ///
    /// # Errors
    ///
    /// This function will return an error if an error occurs during table creation.
    pub fn new(connection: rusqlite::Connection) -> rusqlite::Result<Self> {
        super::create_tables(&connection)?;
        Ok(RusqliteBlockingUrlFetcherCache { connection })
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

    fn fetch_redirect_url_blocking(&self, url: url::Url) -> Result<Option<url::Url>, Self::Error> {
        match super::fetch_redirect_url(&self.connection, &url) {
            Ok(Ok(FetchReturn::Found(value))) => Ok(value),
            Ok(Ok(FetchReturn::NotThere)) => Err(CacheError::NoCacheValue),
            Ok(Err(err)) => Err(CacheError::CacheFetchError(Box::new(
                BlockingFetchError::InvalidUrlError(err),
            ))),
            Err(err) => Err(CacheError::CacheFetchError(Box::new(
                BlockingFetchError::SQLError(err),
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

#[cfg(test)]
mod test {
    use rusqlite::Connection;
    use url::Url;

    use crate::url::url_fetcher::{
        BlockingUrlFetcher,
        cache::{BlockingUrlFetcherCache, CacheError},
    };

    use super::RusqliteBlockingUrlFetcherCache;

    fn setup_cache() -> anyhow::Result<RusqliteBlockingUrlFetcherCache> {
        let c = Connection::open_in_memory()?;
        let cache = RusqliteBlockingUrlFetcherCache::new(c)?;
        Ok(cache)
    }

    #[test]
    fn redirect() -> anyhow::Result<()> {
        let cache = setup_cache()?;

        let url1 = Url::parse("https://google.com/")?;
        let url2 = Url::parse("https://bing.com/")?;
        let url3 = Url::parse("https://yahoo.com/")?;

        assert!(matches!(
            cache.fetch_redirect_url_blocking(url1.clone()),
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_redirect_url_blocking(url2.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_redirect_blocking(url1.clone(), Some(url3.clone()))?;

        assert_eq!(
            cache.fetch_redirect_url_blocking(url1.clone())?,
            Some(url3.clone())
        );
        assert!(matches!(
            cache.fetch_redirect_url_blocking(url2.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_redirect_blocking(url2.clone(), None)?;

        assert_eq!(
            cache.fetch_redirect_url_blocking(url1.clone())?,
            Some(url3.clone())
        );
        assert_eq!(cache.fetch_redirect_url_blocking(url2.clone())?, None);

        cache.store_redirect_blocking(url1.clone(), None)?;

        assert_eq!(cache.fetch_redirect_url_blocking(url1.clone())?, None);
        assert_eq!(cache.fetch_redirect_url_blocking(url2.clone())?, None);

        Ok(())
    }

    #[test]
    fn result() -> anyhow::Result<()> {
        let cache = setup_cache()?;

        let url1 = Url::parse("https://google.com/")?;
        let url2 = Url::parse("https://bing.com/")?;

        let val1 = "THISISARANDOMID";
        let val2 = "ANOTHERRANDOMID";

        assert!(matches!(
            cache.fetch_result_blocking(url1.clone()),
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_result_blocking(url2.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result_blocking(url1.clone(), val1.into())?;

        assert_eq!(cache.fetch_result_blocking(url1.clone())?, val1.into());
        assert!(matches!(
            cache.fetch_result_blocking(url2.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result_blocking(url2.clone(), val2.into())?;

        assert_eq!(cache.fetch_result_blocking(url1.clone())?, val1.into());
        assert_eq!(cache.fetch_result_blocking(url2.clone())?, val2.into());

        cache.store_result_blocking(url1.clone(), val2.into())?;

        assert_eq!(cache.fetch_result_blocking(url1.clone())?, val2.into());
        assert_eq!(cache.fetch_result_blocking(url2.clone())?, val2.into());

        Ok(())
    }

    #[test]
    fn overlap() -> anyhow::Result<()> {
        let cache = setup_cache()?;

        let url1 = Url::parse("https://google.com/")?;
        let url2 = Url::parse("https://bing.com/")?;

        let val1 = "THISISARANDOMID";

        assert!(matches!(
            cache.fetch_redirect_url_blocking(url1.clone()),
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_result_blocking(url1.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_redirect_blocking(url1.clone(), Some(url2.clone()))?;

        assert_eq!(
            cache.fetch_redirect_url_blocking(url1.clone())?,
            Some(url2.clone())
        );
        assert!(matches!(
            cache.fetch_result_blocking(url1.clone()),
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result_blocking(url1.clone(), val1.into())?;

        assert_eq!(
            cache.fetch_redirect_url_blocking(url1.clone())?,
            Some(url2.clone())
        );
        assert_eq!(cache.fetch_result_blocking(url1.clone())?, val1.into());

        cache.store_redirect_blocking(url1.clone(), Some(url1.clone()))?;

        assert_eq!(
            cache.fetch_redirect_url_blocking(url1.clone())?,
            Some(url1.clone())
        );
        assert_eq!(cache.fetch_result_blocking(url1.clone())?, val1.into());

        Ok(())
    }
}
