use std::error::Error;

use async_trait::async_trait;
use url::Url;

use super::UrlFetcher;

pub struct MapErrUrlFetcher<F, Map, E> {
    inner: F,
    mapper: Map,
    _phantom: std::marker::PhantomData<E>,
}

#[async_trait]
impl<F, Map, E> UrlFetcher for MapErrUrlFetcher<F, Map, E>
where
    F: UrlFetcher + Send + Sync,
    Map: Fn(F::Error) -> E + Send + Sync,
    E: Error + Send + Sync + 'static,
{
    type Error = E;

    async fn fetch_redirect_url(&self, url: Url) -> Result<Option<Url>, Self::Error> {
        self.inner
            .fetch_redirect_url(url)
            .await
            .map_err(&self.mapper)
    }

    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Self::Error> {
        self.inner.fetch_result(url).await.map_err(&self.mapper)
    }
}

pub trait UrlFetcherErrExt: UrlFetcher + Sized {
    fn map_err<E, Map>(self, mapper: Map) -> MapErrUrlFetcher<Self, Map, E>
    where
        Map: Fn(Self::Error) -> E + Send + Sync,
        E: Error + Send + Sync + 'static;
}

impl<F> UrlFetcherErrExt for F
where
    F: UrlFetcher + Sized,
{
    fn map_err<E, Map>(self, mapper: Map) -> MapErrUrlFetcher<Self, Map, E>
    where
        Map: Fn(Self::Error) -> E + Send + Sync,
        E: Error + Send + Sync + 'static,
    {
        MapErrUrlFetcher {
            inner: self,
            mapper,
            _phantom: std::marker::PhantomData,
        }
    }
}
