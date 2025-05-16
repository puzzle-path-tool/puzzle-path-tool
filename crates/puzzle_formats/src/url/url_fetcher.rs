use std::error::Error;

use async_trait::async_trait;
use url::Url;

pub mod reqwest;

#[async_trait]
pub trait UrlFetcher {
    async fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error + Send + Sync>>;
    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Box<dyn Error + Send + Sync>>;
}

pub trait BlockingUrlFetcher {
    #[allow(clippy::missing_errors_doc)]
    fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error>>;
    #[allow(clippy::missing_errors_doc)]
    fn fetch_result(&self, url: Url) -> Result<Box<str>, Box<dyn Error>>;
}
