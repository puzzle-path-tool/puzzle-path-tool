use std::error::Error;

use async_trait::async_trait;
use url::Url;

use super::UrlFetcher;

#[derive(Debug, thiserror::Error)]
pub enum CacheError<E> {
    #[error("No Cache Value")]
    NoCacheValue,
    #[error("Error fetching Cache: {0}")]
    CacheFetchError(#[from] Box<E>),
}

#[async_trait]
pub trait UrlFetcherCache: UrlFetcher<Error = CacheError<Self::FetchError>> {
    type FetchError: Error + Send + Sync + 'static;
    type StoreError: Error + Send + Sync + 'static;

    #[allow(clippy::missing_errors_doc)]
    async fn store_redirect(&self, url: Url, value: Option<Url>) -> Result<(), Self::StoreError>;
    #[allow(clippy::missing_errors_doc)]
    async fn store_result(&self, url: Url, value: Box<str>) -> Result<(), Self::StoreError>;
}

pub struct CachedFetcher<C, F> {
    cache: C,
    fetcher: F,
}

#[derive(Debug, thiserror::Error)]
pub enum CachedFetcherError<CF, CS, F> {
    #[error("Error fetching Cache: {0}")]
    CacheFetchError(Box<CF>),
    #[error("Error storing Cache: {0}")]
    CacheStoreError(Box<CS>),
    #[error("Error fetching: {0}")]
    FetchError(Box<F>),
}

#[async_trait]
impl<C, F> UrlFetcher for CachedFetcher<C, F>
where
    C: UrlFetcherCache + Send + Sync,
    F: UrlFetcher + Send + Sync,
{
    type Error = CachedFetcherError<C::FetchError, C::StoreError, F::Error>;

    async fn fetch_redirect_url(&self, url: Url) -> Result<Option<Url>, Self::Error> {
        match self.cache.fetch_redirect_url(url.clone()).await {
            Ok(value) => return Ok(value),
            Err(CacheError::CacheFetchError(err)) => {
                return Err(CachedFetcherError::CacheFetchError(err));
            }
            Err(CacheError::NoCacheValue) => {}
        }

        let value = self
            .fetcher
            .fetch_redirect_url(url.clone())
            .await
            .map_err(|err| CachedFetcherError::FetchError(err.into()))?;

        self.cache
            .store_redirect(url, value.clone())
            .await
            .map_err(|err| CachedFetcherError::CacheStoreError(err.into()))?;

        Ok(value)
    }

    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Self::Error> {
        match self.cache.fetch_result(url.clone()).await {
            Ok(value) => return Ok(value),
            Err(CacheError::CacheFetchError(err)) => {
                return Err(CachedFetcherError::CacheFetchError(err));
            }
            Err(CacheError::NoCacheValue) => {}
        }

        let value = self
            .fetcher
            .fetch_result(url.clone())
            .await
            .map_err(|err| CachedFetcherError::FetchError(err.into()))?;

        self.cache
            .store_result(url, value.clone())
            .await
            .map_err(|err| CachedFetcherError::CacheStoreError(err.into()))?;

        Ok(value)
    }
}
