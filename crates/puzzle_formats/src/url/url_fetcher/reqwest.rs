#[cfg(any(feature = "reqwest", feature = "reqwest-blocking"))]
mod reqwest_fetcher {
    pub use crate::url::url_fetcher::BlockingUrlFetcher;
    pub use crate::url::url_fetcher::UrlFetcher;
    pub use async_trait::async_trait;
    pub use std::error::Error;
    pub use url::Url;

    pub struct ReqwestUrlFetcher {}

    impl ReqwestUrlFetcher {
        #[must_use]
        pub fn new() -> Self {
            ReqwestUrlFetcher {}
        }
    }

    impl Default for ReqwestUrlFetcher {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(feature = "reqwest")]
    #[async_trait]
    impl UrlFetcher for ReqwestUrlFetcher {
        async fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error + Send + Sync>> {
            let client = reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client.request(reqwest::Method::GET, url).send().await?;

            Ok(response.url().to_owned())
        }

        async fn fetch_result(&self, url: Url) -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
            let client = reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()?;

            let response = client.request(reqwest::Method::GET, url).send().await?;

            let text = response.text().await?;

            Ok(text.into_boxed_str())
        }
    }

    #[cfg(feature = "reqwest-blocking")]
    impl BlockingUrlFetcher for ReqwestUrlFetcher {
        fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error>> {
            let client = reqwest::blocking::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client.request(reqwest::Method::GET, url).send()?;

            Ok(response.url().to_owned())
        }

        fn fetch_result(&self, url: Url) -> Result<Box<str>, Box<dyn Error>> {
            let client = reqwest::blocking::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::limited(10))
                .build()?;

            let response = client.request(reqwest::Method::GET, url).send()?;

            let text = response.text()?;

            Ok(text.into_boxed_str())
        }
    }
}

#[cfg(any(feature = "reqwest", feature = "reqwest-blocking"))]
pub use reqwest_fetcher::*;
