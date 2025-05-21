#![cfg(any(feature = "reqwest", feature = "reqwest-blocking"))]
pub struct ReqwestUrlFetcher {}

impl ReqwestUrlFetcher {
    #[must_use]
    pub const fn new() -> Self {
        ReqwestUrlFetcher {}
    }
}

impl Default for ReqwestUrlFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Location Header is malformed: {0}")]
    MalformedLocation(#[from] reqwest::header::ToStrError),
    #[error("Location Header is malformed: {0}")]
    InvalidLocationUrl(#[source] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "reqwest")]
mod reqwest_async {
    use crate::url::url_fetcher::UrlFetcher;
    use async_trait::async_trait;
    use url::Url;

    use super::ReqwestUrlFetcher;

    #[async_trait]
    impl UrlFetcher for ReqwestUrlFetcher {
        type Error = super::Error;

        async fn fetch_redirect_url(&self, url: Url) -> super::Result<Option<Url>> {
            let client = reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client.get(url.clone()).send().await?;

            let location = response.headers().get(reqwest::header::LOCATION);

            let Some(location) = location else {
                return Ok(None);
            };

            let url = url
                .join(location.to_str()?)
                .map_err(super::Error::InvalidLocationUrl)?;

            Ok(Some(url))
        }

        async fn fetch_result(&self, url: Url) -> super::Result<Box<str>> {
            let client = reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()?;

            let response = client.get(url).send().await?;

            let text = response.text().await?;

            Ok(text.into_boxed_str())
        }
    }
}

#[cfg(feature = "reqwest-blocking")]
mod reqwest_blocking {
    use crate::url::url_fetcher::BlockingUrlFetcher;
    use url::Url;

    use super::ReqwestUrlFetcher;

    impl BlockingUrlFetcher for ReqwestUrlFetcher {
        type Error = super::Error;

        fn fetch_redirect_url(&self, url: Url) -> super::Result<Option<Url>> {
            let client = reqwest::blocking::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client.get(url.clone()).send()?;

            let location = response.headers().get(reqwest::header::LOCATION);

            let Some(location) = location else {
                return Ok(None);
            };

            let url = url
                .join(location.to_str()?)
                .map_err(super::Error::InvalidLocationUrl)?;

            Ok(Some(url))
        }

        fn fetch_result(&self, url: Url) -> super::Result<Box<str>> {
            let client = reqwest::blocking::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()?;

            let response = client.get(url).send()?;

            let text = response.text()?;

            Ok(text.into_boxed_str())
        }
    }
}
