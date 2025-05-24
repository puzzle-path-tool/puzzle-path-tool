use std::error::Error;

use async_trait::async_trait;
use url::Url;

pub mod cache;
pub mod map_err;
pub mod reqwest;
pub mod rusqlite;
pub mod tokio;

/// Async fetcher, used to define fetching behaviour.
#[async_trait]
pub trait UrlFetcher {
    type Error: Error + Send + Sync + 'static;

    /// Fetch the redirect when requesting a Url.
    /// Returns `None`, when no redirect occurred.
    ///
    /// # Errors
    ///
    /// Errors are implementation-specific, and their behavior depends on the underlying fetcher.
    async fn fetch_redirect_url(&self, url: Url) -> Result<Option<Url>, Self::Error>;

    /// Fetch the Result of a request to a Url.
    ///
    /// # Errors
    ///
    /// Errors are implementation-specific, and their behavior depends on the underlying fetcher.
    async fn fetch_result(&self, url: Url) -> Result<Box<str>, Self::Error>;
}

/// Blocking fetcher, used to define fetching behaviour.
pub trait BlockingUrlFetcher {
    type Error: Error + 'static;

    /// Fetch the redirect when requesting a Url.
    /// Returns `None`, when no redirect occurred.
    ///
    /// # Errors
    ///
    /// Errors are implementation-specific, and their behavior depends on the underlying fetcher.
    fn fetch_redirect_url_blocking(&self, url: Url) -> Result<Option<Url>, Self::Error>;

    /// Fetch the Result of a request to a Url.
    ///
    /// # Errors
    ///
    /// Errors are implementation-specific, and their behavior depends on the underlying fetcher.
    fn fetch_result_blocking(&self, url: Url) -> Result<Box<str>, Self::Error>;
}
