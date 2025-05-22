use std::error::Error;

use url::Url;

use super::{
    resolved_url::ResolvedUrl,
    url_fetcher::{BlockingUrlFetcher, UrlFetcher},
};

pub struct UnresolvedUrl {
    inner: UnresolvedUrlInner,
}

#[derive(Debug, thiserror::Error)]
pub enum ResolutionError<E: Error> {
    #[error("Fetcher Error: {0}")]
    Fetcher(#[from] E),
    #[error("Puzzle Id could not be inserted into Url: {0}")]
    MalformedId(#[source] url::ParseError),
}

pub(crate) enum UnresolvedUrlInner {
    // https://f-puzzles.com/?id=TINYURLID
    // get the redirect from https://tinyurl.com/TINYURLID OR the url itself (f-puzzles does not do any safety check, for the redirect)
    FPuzzles(Box<str>),
    // get the correct id as text response from https://sudokupad.app/api/puzzle/SHORTID
    SudokuPad(Box<str>),
    // Just resolve Redirect
    Unknown(Box<Url>),
}

#[derive(Debug, Default)]
pub struct ResolutionOptions {}

impl UnresolvedUrl {
    pub(crate) fn new(inner: UnresolvedUrlInner) -> Self {
        Self { inner }
    }
}

impl UnresolvedUrl {
    #[allow(clippy::missing_errors_doc)]
    pub async fn resolve<F, E>(
        &self,
        fetcher: &F,
        _options: &ResolutionOptions,
    ) -> Result<ResolvedUrl, ResolutionError<E>>
    where
        F: UrlFetcher<Error = E>,
        E: Error + Send + Sync + 'static,
    {
        use ResolutionStep as R;

        match self
            .resolution_step()
            .map_err(ResolutionError::MalformedId)?
        {
            R::FetchRedirectUrl(url) => {
                let _value = fetcher.fetch_redirect_url(url.clone()).await?;
            }
            R::FetchResult(url) => {
                let _value = fetcher.fetch_result(url).await?;
            }
        }

        todo!(
            "Only one value will be requested and processed based on url and maybe even options param"
        )
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn resolve_blocking<F, E>(
        &self,
        fetcher: &F,
        _options: &ResolutionOptions,
    ) -> Result<ResolvedUrl, ResolutionError<E>>
    where
        F: BlockingUrlFetcher<Error = E>,
        E: Error + 'static,
    {
        use ResolutionStep as R;

        match self
            .resolution_step()
            .map_err(ResolutionError::MalformedId)?
        {
            R::FetchRedirectUrl(url) => {
                let _value = fetcher.fetch_redirect_url_blocking(url.clone())?;
            }
            R::FetchResult(url) => {
                let _value = fetcher.fetch_result_blocking(url)?;
            }
        }

        todo!(
            "Only one value will be requested and processed based on url and maybe even options param"
        )
    }

    fn resolution_step(&self) -> Result<ResolutionStep, url::ParseError> {
        use ResolutionStep as R;
        use UnresolvedUrlInner as U;

        match &self.inner {
            U::FPuzzles(id) => {
                let url = Url::parse(format!("https://tinyurl.com/{id}").as_str())?;
                Ok(R::FetchRedirectUrl(url))
            }
            U::SudokuPad(id) => {
                let url = Url::parse(format!("https://sudokupad.app/api/puzzle/{id}").as_str())?;
                Ok(R::FetchResult(url))
            }
            U::Unknown(url) => Ok(R::FetchRedirectUrl(*url.clone())),
        }
    }
}

enum ResolutionStep {
    FetchResult(Url),
    FetchRedirectUrl(Url),
}
