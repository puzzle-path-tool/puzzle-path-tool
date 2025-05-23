use std::error::Error;

use async_trait::async_trait;
use url::Url;

pub mod cache;
pub mod map_err;
pub mod reqwest;
pub mod rusqlite;
pub mod tokio;
pub mod tokio_rusqlite;

#[async_trait]
pub trait UrlFetcher {
    type Error: Error + Send + Sync + 'static;

    async fn fetch_redirect_url(&self, url: Url) -> Result<Option<Url>, Self::Error>;
    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Self::Error>;
}

pub trait BlockingUrlFetcher {
    type Error: Error + 'static;

    #[allow(clippy::missing_errors_doc)]
    fn fetch_redirect_url_blocking(&self, url: Url) -> Result<Option<Url>, Self::Error>;
    #[allow(clippy::missing_errors_doc)]
    fn fetch_result_blocking(&self, url: Url) -> Result<Box<str>, Self::Error>;
}
