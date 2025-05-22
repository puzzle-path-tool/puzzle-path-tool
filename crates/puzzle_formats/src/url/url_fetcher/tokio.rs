use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use tokio::task::JoinError;
use url::Url;

use super::{
    BlockingUrlFetcher, UrlFetcher,
    cache::{CacheError, UrlFetcherCache},
};

pub struct TokioUrlFetcher<F> {
    inner: Arc<F>,
}

#[derive(Debug, thiserror::Error)]
pub enum TokioFetcherError<E> {
    #[error("Error joining Tokio Task: {0}")]
    JoinError(#[from] Box<JoinError>),
    #[error("Error fetching: {0}")]
    FetchError(Box<E>),
}

#[async_trait]
impl<F> UrlFetcher for TokioUrlFetcher<F>
where
    F: BlockingUrlFetcher + Send + Sync + 'static,
    F::Error: Error + Send + Sync,
{
    type Error = TokioFetcherError<F::Error>;

    async fn fetch_redirect_url(&self, url: Url) -> Result<Option<Url>, Self::Error> {
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || inner.fetch_redirect_url_blocking(url))
            .await
            .map_err(|err| TokioFetcherError::JoinError(err.into()))?
            .map_err(|err| TokioFetcherError::FetchError(err.into()))
    }

    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Self::Error> {
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || inner.fetch_result_blocking(url))
            .await
            .map_err(|err| TokioFetcherError::JoinError(err.into()))?
            .map_err(|err| TokioFetcherError::FetchError(err.into()))
    }
}

pub trait UrlFetcherTokioExt: BlockingUrlFetcher + Sized {
    fn into_async(self) -> TokioUrlFetcher<Self>;
}

impl<F> UrlFetcherTokioExt for F
where
    F: BlockingUrlFetcher + Sized + Send + Sync + 'static,
    F::Error: Error + Send + Sync,
{
    fn into_async(self) -> TokioUrlFetcher<Self> {
        TokioUrlFetcher { inner: self.into() }
    }
}

#[async_trait]
impl<F> UrlFetcherCache for TokioUrlFetcher<F>
where
    TokioUrlFetcher<F>: UrlFetcher<Error = CacheError<F::FetchError>>,
    F: BlockingUrlFetcher + UrlFetcherCache + Send + Sync,
{
    type FetchError = F::FetchError;
    type StoreError = F::StoreError;

    #[allow(clippy::missing_errors_doc)]
    async fn store_redirect(&self, url: Url, value: Option<Url>) -> Result<(), Self::StoreError> {
        self.inner.store_redirect(url, value).await
    }
    #[allow(clippy::missing_errors_doc)]
    async fn store_result(&self, url: Url, value: Box<str>) -> Result<(), Self::StoreError> {
        self.inner.store_result(url, value).await
    }
}
