use std::error::Error;

use async_trait::async_trait;
use url::Url;

use super::{BlockingUrlFetcher, UrlFetcher};

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

#[derive(Debug)]
struct CacheResultTable {
    request_url: Url,
    response_value: Box<str>,
}

#[derive(Debug)]
struct CacheRedirectTable {
    request_url: Url,
    redirect_url: Option<Url>,
}

pub trait BlockingUrlFetcherCache:
    BlockingUrlFetcher<Error = CacheError<Self::FetchError>>
{
    type FetchError: Error + 'static;
    type StoreError: Error + 'static;

    #[allow(clippy::missing_errors_doc)]
    fn store_redirect_blocking(&self, url: Url, value: Option<Url>)
    -> Result<(), Self::StoreError>;
    #[allow(clippy::missing_errors_doc)]
    fn store_result_blocking(&self, url: Url, value: Box<str>) -> Result<(), Self::StoreError>;
}

pub struct CachedFetcher<C, F> {
    cache: C,
    fetcher: F,
}

impl<C, F> CachedFetcher<C, F>
where
    C: UrlFetcherCache + Send + Sync,
    F: UrlFetcher + Send + Sync,
{
    pub fn new(cache: C, fetcher: F) -> Self {
        Self { cache, fetcher }
    }
}

impl<C, F> CachedFetcher<C, F>
where
    C: BlockingUrlFetcherCache,
    F: BlockingUrlFetcher,
{
    pub fn blocking(cache: C, fetcher: F) -> Self {
        Self { cache, fetcher }
    }
}

pub trait UrlFetcherCacheExt: UrlFetcher + Sized {
    fn with_cache<C>(self, cache: C) -> CachedFetcher<C, Self>
    where
        C: UrlFetcherCache + Send + Sync;
}

pub trait BlockingUrlFetcherCacheExt: BlockingUrlFetcher + Sized {
    fn with_cache_blocking<C>(self, cache: C) -> CachedFetcher<C, Self>
    where
        C: BlockingUrlFetcherCache;
}

impl<F> UrlFetcherCacheExt for F
where
    F: UrlFetcher + Send + Sync,
{
    fn with_cache<C>(self, cache: C) -> CachedFetcher<C, Self>
    where
        C: UrlFetcherCache + Send + Sync,
    {
        CachedFetcher::new(cache, self)
    }
}

impl<F> BlockingUrlFetcherCacheExt for F
where
    F: BlockingUrlFetcher,
{
    fn with_cache_blocking<C>(self, cache: C) -> CachedFetcher<C, Self>
    where
        C: BlockingUrlFetcherCache,
    {
        CachedFetcher::blocking(cache, self)
    }
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

impl<C, F> BlockingUrlFetcher for CachedFetcher<C, F>
where
    C: BlockingUrlFetcherCache,
    F: BlockingUrlFetcher,
{
    type Error = CachedFetcherError<C::FetchError, C::StoreError, F::Error>;

    fn fetch_redirect_url_blocking(&self, url: Url) -> Result<Option<Url>, Self::Error> {
        match self.cache.fetch_redirect_url_blocking(url.clone()) {
            Ok(value) => return Ok(value),
            Err(CacheError::CacheFetchError(err)) => {
                return Err(CachedFetcherError::CacheFetchError(err));
            }
            Err(CacheError::NoCacheValue) => {}
        }

        let value = self
            .fetcher
            .fetch_redirect_url_blocking(url.clone())
            .map_err(|err| CachedFetcherError::FetchError(err.into()))?;

        self.cache
            .store_redirect_blocking(url, value.clone())
            .map_err(|err| CachedFetcherError::CacheStoreError(err.into()))?;

        Ok(value)
    }

    fn fetch_result_blocking(&self, url: Url) -> Result<Box<str>, Self::Error> {
        match self.cache.fetch_result_blocking(url.clone()) {
            Ok(value) => return Ok(value),
            Err(CacheError::CacheFetchError(err)) => {
                return Err(CachedFetcherError::CacheFetchError(err));
            }
            Err(CacheError::NoCacheValue) => {}
        }

        let value = self
            .fetcher
            .fetch_result_blocking(url.clone())
            .map_err(|err| CachedFetcherError::FetchError(err.into()))?;

        self.cache
            .store_result_blocking(url, value.clone())
            .map_err(|err| CachedFetcherError::CacheStoreError(err.into()))?;

        Ok(value)
    }
}
