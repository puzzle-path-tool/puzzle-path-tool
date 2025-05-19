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

#[cfg(feature = "reqwest")]
mod reqwest_async {
    use crate::url::url_fetcher::UrlFetcher;
    use async_trait::async_trait;
    use std::error::Error;
    use url::Url;

    use super::ReqwestUrlFetcher;

    #[async_trait]
    impl UrlFetcher for ReqwestUrlFetcher {
        async fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error + Send + Sync>> {
            let client = reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client
                .request(reqwest::Method::GET, url.clone())
                .send()
                .await?;

            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .ok_or("No Redirect")?;
            let url = url.join(location.to_str()?)?;

            Ok(url)
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
}

#[cfg(feature = "reqwest-blocking")]
mod reqwest_blocking {
    use crate::url::url_fetcher::BlockingUrlFetcher;
    use std::error::Error;
    use url::Url;

    use super::ReqwestUrlFetcher;

    impl BlockingUrlFetcher for ReqwestUrlFetcher {
        fn fetch_redirect_url(&self, url: Url) -> Result<Url, Box<dyn Error>> {
            let client = reqwest::blocking::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;

            let response = client.request(reqwest::Method::GET, url.clone()).send()?;

            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .ok_or("No Redirect")?;
            let url = url.join(location.to_str()?)?;

            Ok(url)
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
