#![cfg(feature = "tokio-rusqlite")]

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
    /// Create a new Instance.
    /// This will also create database tables via the connection.
    ///
    /// # Errors
    ///
    /// This function will return an error if an error occurs during table creation.
    pub async fn new(connection: tokio_rusqlite::Connection) -> tokio_rusqlite::Result<Self> {
        connection
            .call(|connection| {
                super::create_tables(connection)?;
                Ok(())
            })
            .await?;
        Ok(RusqliteUrlFetcherCache { connection })
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

    async fn store_result(&self, url: url::Url, value: Box<str>) -> Result<(), Self::StoreError> {
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

    async fn fetch_redirect_url(&self, url: url::Url) -> Result<Option<url::Url>, Self::Error> {
        let result = self
            .connection
            .call(move |connection| {
                let fetch_return = super::fetch_redirect_url(connection, &url)?;
                Ok(fetch_return)
            })
            .await;

        match result {
            Ok(Ok(FetchReturn::Found(value))) => Ok(value),
            Ok(Ok(FetchReturn::NotThere)) => Err(CacheError::NoCacheValue),
            Ok(Err(err)) => Err(CacheError::CacheFetchError(Box::new(
                FetchError::InvalidUrlError(err),
            ))),
            Err(err) => Err(CacheError::CacheFetchError(Box::new(FetchError::SQLError(
                err,
            )))),
        }
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

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod test {
    use tokio_rusqlite::Connection;
    use url::Url;

    use crate::url::url_fetcher::{
        UrlFetcher,
        cache::{CacheError, UrlFetcherCache},
    };

    use super::RusqliteUrlFetcherCache;

    async fn setup_cache() -> RusqliteUrlFetcherCache {
        let c = Connection::open_in_memory().await.unwrap();
        RusqliteUrlFetcherCache::new(c).await.unwrap()
    }

    #[tokio::test]
    async fn redirect() {
        let cache = setup_cache().await;

        let url1 = Url::parse("https://google.com/").unwrap();
        let url2 = Url::parse("https://bing.com/").unwrap();
        let url3 = Url::parse("https://yahoo.com/").unwrap();

        assert!(matches!(
            cache.fetch_redirect_url(url1.clone()).await,
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_redirect_url(url2.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache
            .store_redirect(url1.clone(), Some(url3.clone()))
            .await
            .unwrap();

        assert_eq!(
            cache.fetch_redirect_url(url1.clone()).await.unwrap(),
            Some(url3.clone())
        );
        assert!(matches!(
            cache.fetch_redirect_url(url2.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache.store_redirect(url2.clone(), None).await.unwrap();

        assert_eq!(
            cache.fetch_redirect_url(url1.clone()).await.unwrap(),
            Some(url3.clone())
        );
        assert_eq!(cache.fetch_redirect_url(url2.clone()).await.unwrap(), None);

        cache.store_redirect(url1.clone(), None).await.unwrap();

        assert_eq!(cache.fetch_redirect_url(url1.clone()).await.unwrap(), None);
        assert_eq!(cache.fetch_redirect_url(url2.clone()).await.unwrap(), None);
    }

    #[tokio::test]
    async fn result() {
        let cache = setup_cache().await;

        let url1 = Url::parse("https://google.com/").unwrap();
        let url2 = Url::parse("https://bing.com/").unwrap();

        let val1 = "THISISARANDOMID";
        let val2 = "ANOTHERRANDOMID";

        assert!(matches!(
            cache.fetch_result(url1.clone()).await,
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_result(url2.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result(url1.clone(), val1.into()).await.unwrap();

        assert_eq!(cache.fetch_result(url1.clone()).await.unwrap(), val1.into());
        assert!(matches!(
            cache.fetch_result(url2.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result(url2.clone(), val2.into()).await.unwrap();

        assert_eq!(cache.fetch_result(url1.clone()).await.unwrap(), val1.into());
        assert_eq!(cache.fetch_result(url2.clone()).await.unwrap(), val2.into());

        cache.store_result(url1.clone(), val2.into()).await.unwrap();

        assert_eq!(cache.fetch_result(url1.clone()).await.unwrap(), val2.into());
        assert_eq!(cache.fetch_result(url2.clone()).await.unwrap(), val2.into());
    }

    #[tokio::test]
    async fn overlap() {
        let cache = setup_cache().await;

        let url1 = Url::parse("https://google.com/").unwrap();
        let url2 = Url::parse("https://bing.com/").unwrap();

        let val1 = "THISISARANDOMID";

        assert!(matches!(
            cache.fetch_redirect_url(url1.clone()).await,
            Err(CacheError::NoCacheValue)
        ));
        assert!(matches!(
            cache.fetch_result(url1.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache
            .store_redirect(url1.clone(), Some(url2.clone()))
            .await
            .unwrap();

        assert_eq!(
            cache.fetch_redirect_url(url1.clone()).await.unwrap(),
            Some(url2.clone())
        );
        assert!(matches!(
            cache.fetch_result(url1.clone()).await,
            Err(CacheError::NoCacheValue)
        ));

        cache.store_result(url1.clone(), val1.into()).await.unwrap();

        assert_eq!(
            cache.fetch_redirect_url(url1.clone()).await.unwrap(),
            Some(url2.clone())
        );
        assert_eq!(cache.fetch_result(url1.clone()).await.unwrap(), val1.into());

        cache
            .store_redirect(url1.clone(), Some(url1.clone()))
            .await
            .unwrap();

        assert_eq!(
            cache.fetch_redirect_url(url1.clone()).await.unwrap(),
            Some(url1.clone())
        );
        assert_eq!(cache.fetch_result(url1.clone()).await.unwrap(), val1.into());
    }
}
