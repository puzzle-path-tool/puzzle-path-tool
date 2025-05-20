use std::{borrow::Cow, error::Error};

use itertools::Itertools;
use serde_json::Value;
use url::Url;
use url_fetcher::{BlockingUrlFetcher, UrlFetcher};

pub mod url_fetcher;

/// .
///
/// # Panics
///
/// Panics if .
#[must_use]
#[deprecated]
pub fn old_decode_url(url: &str) -> Value {
    let bytes = lz_str::decompress_from_base64(url).unwrap_or_else(|| panic!("Could not decode"));
    let json_str = String::from_utf16(&bytes).unwrap_or_else(|e| panic!("No Utf8: {e}"));

    serde_json::from_str(&json_str).unwrap_or_else(|e| panic!("No Valid JSON: {e}"))
}

pub struct PuzzleFormat {}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid Url Scheme: {0}")]
    InvalidScheme(Box<str>),
    #[error("No Domain could be extracted from Url: {0}")]
    NoDomain(Box<Url>),
    #[error("Unknown Page at Url: {0}")]
    UnknownPage(Box<Url>),
    #[error("No Id could be extracted from Url: {0}")]
    MissingId(Box<Url>),
}

pub enum UrlValue {
    Resolved(ResolvedUrl),
    Unresolved(UnresolvedUrl),
}

impl UrlValue {
    fn resolved(inner: ResolvedUrlInner) -> Self {
        Self::Resolved(ResolvedUrl::new(inner))
    }

    fn unresolved(inner: UnresolvedUrlInner) -> Self {
        Self::Unresolved(UnresolvedUrl::new(inner))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn parse(url: &Url) -> Result<Self, ParseError> {
        if !matches!(url.scheme(), "http" | "https") {
            return Err(ParseError::InvalidScheme(url.scheme().into()));
        }

        let Some(domain) = url.domain() else {
            return Err(ParseError::NoDomain(url.clone().into()));
        };

        let Some(mut segments) = url.path_segments() else {
            return Err(ParseError::NoDomain(url.clone().into()));
        };

        let mut query_pairs = url.query_pairs();

        match domain {
            "sudokupad.app" | "alpha.sudokupad.app" | "beta.sudokupad.app" => {
                let puzzleid = query_pairs.find_map(|(k, v)| (k == "puzzleid").then_some(v));

                let puzzleid = puzzleid.unwrap_or_else(|| {
                    let mut segments = segments.peekable();

                    if segments.peek() == Some(&"sudoku") {
                        segments.next();
                    }

                    let puzzleid = segments.join("/");
                    Cow::Owned(puzzleid)
                });

                if let Some(puzzleid) = puzzleid
                    .strip_prefix("fpuzzles")
                    .or_else(|| puzzleid.strip_prefix("fpuz"))
                {
                    return Ok(Self::resolved(ResolvedUrlInner::SudokuPad(
                        SudokuPadFullUrl::FPuz(puzzleid.into()),
                    )));
                }

                if let Some(puzzleid) = puzzleid
                    .strip_prefix("scl")
                    .or_else(|| puzzleid.strip_prefix("ctc"))
                {
                    return Ok(Self::resolved(ResolvedUrlInner::SudokuPad(
                        SudokuPadFullUrl::Scl(puzzleid.into()),
                    )));
                }

                if let Some(puzzleid) = puzzleid.strip_prefix("scf") {
                    return Ok(Self::resolved(ResolvedUrlInner::SudokuPad(
                        SudokuPadFullUrl::Scf(puzzleid.into()),
                    )));
                }

                Ok(Self::unresolved(UnresolvedUrlInner::SudokuPad(
                    puzzleid.into(),
                )))
            }
            "f-puzzles.com" | "www.f-puzzles.com" => {
                if segments.next().is_some() {
                    return Err(ParseError::UnknownPage(url.clone().into()));
                }

                let Some((k, v)) = query_pairs.next() else {
                    return Err(ParseError::MissingId(url.clone().into()));
                };

                match k.as_ref() {
                    "id" => {
                        let puzzleid = v;
                        Ok(Self::unresolved(UnresolvedUrlInner::FPuzzles(
                            puzzleid.into_owned().into(),
                        )))
                    }
                    "load" => {
                        let puzzleid = v;
                        Ok(Self::resolved(ResolvedUrlInner::FPuzzles(
                            puzzleid.into_owned().into(),
                        )))
                    }
                    _k => Err(ParseError::MissingId(url.clone().into())),
                }
            }
            "sudokumaker.app" => {
                let puzzleid = query_pairs.find_map(|(k, v)| (k == "puzzle").then_some(v));

                let Some(puzzleid) = puzzleid else {
                    return Err(ParseError::MissingId(url.clone().into()));
                };

                Ok(Self::resolved(ResolvedUrlInner::SudokuMaker(
                    puzzleid.into_owned().into_boxed_str(),
                )))
            }
            "swaroopg92.github.io" => {
                if !(segments.next() == Some("penpa-edit") && segments.next().is_none()) {
                    return Ok(Self::unresolved(UnresolvedUrlInner::Unknown(
                        url.clone().into(),
                    )));
                }

                Ok(Self::resolved(ResolvedUrlInner::Penpa(url.clone().into())))
            }
            _ => Ok(Self::unresolved(UnresolvedUrlInner::Unknown(Box::new(
                url.clone(),
            )))),
        }
    }
}

pub struct ResolvedUrl {
    inner: ResolvedUrlInner,
}

impl ResolvedUrl {
    fn new(inner: ResolvedUrlInner) -> Self {
        Self { inner }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn decode(&self) -> Result<PuzzleFormat, Box<dyn Error>> {
        todo!()
    }
}

enum ResolvedUrlInner {
    FPuzzles(Box<str>),
    SudokuPad(SudokuPadFullUrl),
    SudokuMaker(Box<str>),
    Penpa(Box<Url>),
}

enum SudokuPadFullUrl {
    Scl(Box<str>),
    Scf(Box<str>),
    FPuz(Box<str>),
}

pub struct UnresolvedUrl {
    inner: UnresolvedUrlInner,
}

#[derive(Debug, Default)]
pub struct ResolutionOptions {}

impl UnresolvedUrl {
    fn new(inner: UnresolvedUrlInner) -> Self {
        Self { inner }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn resolve<F>(
        &self,
        fetcher: &F,
        _options: &ResolutionOptions,
    ) -> Result<ResolvedUrl, Box<dyn Error + Send + Sync>>
    where
        F: UrlFetcher,
    {
        use ResolutionStep as R;

        match self.resolution_step()? {
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

    #[allow(clippy::expect_used)]
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn resolve_blocking<F>(
        &self,
        fetcher: &F,
        _options: &ResolutionOptions,
    ) -> Result<ResolvedUrl, Box<dyn Error>>
    where
        F: BlockingUrlFetcher,
    {
        use ResolutionStep as R;

        match self.resolution_step().expect("msg") {
            R::FetchRedirectUrl(url) => {
                let _value = fetcher.fetch_redirect_url(url.clone())?;
            }
            R::FetchResult(url) => {
                let _value = fetcher.fetch_result(url)?;
            }
        }

        todo!(
            "Only one value will be requested and processed based on url and maybe even options param"
        )
    }

    fn resolution_step(&self) -> Result<ResolutionStep, Box<dyn Error + Send + Sync>> {
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

enum UnresolvedUrlInner {
    // https://f-puzzles.com/?id=TINYURLID
    // get the redirect from https://tinyurl.com/TINYURLID OR the url itself (f-puzzles does not do any safety check, for the redirect)
    FPuzzles(Box<str>),
    // get the correct id as text response from https://sudokupad.app/api/puzzle/SHORTID
    SudokuPad(Box<str>),
    // Just resolve Redirect
    Unknown(Box<Url>),
}
