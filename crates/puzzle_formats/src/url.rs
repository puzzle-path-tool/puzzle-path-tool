use std::{borrow::Cow, error::Error};

use itertools::Itertools;
use serde_json::Value;
use url::Url;
use url_fetcher::{BlockingUrlFetcher, UrlFetcher};

pub mod url_fetcher;

pub enum FPuzzlesURL {
    // https://[www].f-puzzles.com/?load=FPUZZLESID
    Normal(String),

    // https://f-puzzles.com/?id=TINYURLID
    // get the redirect from https://tinyurl.com/TINYURLID OR the url itself (f-puzzles does not do any safety check, for the redirect)
    Shortened(String),
}

pub enum SudokuPadUrl {
    // https://sudokupad.app/ANYTHING?puzzleid=ANYSUDOKUPADID
    // https://sudokupad.app/sudoku/ANYSUDOKUPADID
    // https://sudokupad.app/ANYSUDOKUPADID

    // ANYSUDOKUPADID = SHORTID // (fpuz|fpuzzles)FPUZZLESID // (scl|ctc)SUDOKUPADID
    Scl(String),
    FPuz(String),

    // get the correct id as text response from https://sudokupad.app/api/puzzle/SHORTID
    Shortened(String),
}

impl SudokuPadUrl {
    fn from_full_id(full_id: &str) -> Self {
        if full_id.len() > 20 {
            SudokuPadUrl::Scl(String::new())
        } else {
            SudokuPadUrl::Shortened(String::new())
        }
    }
}

pub enum PuzzleUrl {
    SudokuPad(SudokuPadUrl),
    FPuzzles(FPuzzlesURL),
    // sudokumaker.app/?puzzle=SUDOKUMAKERID
    SudokuMakerURL(String),
    // https://swaroopg92.github.io/penpa-edit/ANY
    PenpaUrl(String),
}

/// .
///
/// # Panics
///
/// Panics if .
#[must_use]
pub fn decode_url(url: &str) -> Value {
    let bytes = lz_str::decompress_from_base64(url).unwrap_or_else(|| panic!("Could not decode"));
    let json_str = String::from_utf16(&bytes).unwrap_or_else(|e| panic!("No Utf8: {e}"));

    serde_json::from_str(&json_str).unwrap_or_else(|e| panic!("No Valid JSON: {e}"))
}

pub struct PuzzleFormat {}
pub enum SomeError {
    SomeError,
}

pub enum UrlValue {
    Resolved(ResolvedUrl),
    Unresolved(UnresolvedUrl),
}

impl UrlValue {
    #[allow(clippy::missing_errors_doc)]
    pub fn parse(url: &Url) -> Result<Self, SomeError> {
        if !matches!(url.scheme(), "http" | "https") {
            return Err(SomeError::SomeError); // Invalid Schema
        }

        let Some(domain) = url.domain() else {
            return Err(SomeError::SomeError); // No Domain
        };

        let Some(mut segments) = url.path_segments() else {
            return Err(SomeError::SomeError); // No Domain
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

                todo!("load: {puzzleid}");

                // The resulting id follows the format
                // SHORTID  or  (fpuz|fpuzzles)FPUZZLESID  or  (scl|ctc)SUDOKUPADID  or  (scf)SCFID
            }
            "f-puzzles.com" | "www.f-puzzles.com" => {
                if let Some(_segment) = segments.next() {
                    return Err(SomeError::SomeError); // Unknown Page
                }

                let Some((k, v)) = query_pairs.next() else {
                    return Err(SomeError::SomeError); // Missing Id
                };

                match k.as_ref() {
                    "id" => {
                        let puzzleid = v;
                        todo!("load Short: {puzzleid}");
                    }
                    "load" => {
                        let puzzleid = v;
                        todo!("load Long: {puzzleid}");
                    }
                    _k => Err(SomeError::SomeError), // Missing Id
                }
            }
            "sudokumaker.app" => {
                let puzzleid = query_pairs.find_map(|(k, v)| (k == "puzzle").then_some(v));

                let Some(puzzleid) = puzzleid else {
                    return Err(SomeError::SomeError); // Missing Id
                };

                todo!("load: {puzzleid}");
            }
            "swaroopg92.github.io" => {
                if !(segments.next() == Some("penpa-edit") && segments.next().is_none()) {
                    return Err(SomeError::SomeError); // Unknown Site
                }

                todo!("Dont load for now");

                // Accepts either query params or fragments formatted the same ["#", "?", "#?", "?#"]
                // Just ignore this without parsing for now
            }
            _ => Err(SomeError::SomeError), // Unknown Site
        }
    }
}

pub enum ResolvedUrl {
    FPuzzles(Box<str>),
    SudokuPad(SudokuPadFullUrl),
    SudokuMaker(Box<str>),
    Penpa(Box<str>),
}

impl ResolvedUrl {
    #[allow(clippy::missing_errors_doc)]
    pub fn decode(&self) -> Result<PuzzleFormat, SomeError> {
        todo!()
    }
}

pub enum SudokuPadFullUrl {
    Scl(Box<str>),
    Scf(Box<str>),
    FPuz(Box<str>),
}

pub enum UnresolvedUrl {
    FPuzzles(Box<str>),
    SudokuPad(Box<str>),
    Unknown(Box<str>),
}

impl UnresolvedUrl {
    #[allow(clippy::missing_errors_doc)]
    pub async fn resolve<F>(&self, fetcher: &F) -> Result<ResolvedUrl, Box<dyn Error + Send + Sync>>
    where
        F: UrlFetcher,
    {
        let url = Url::parse("https://localhost:8080")?; // https://tinyurl.com/2b5dwuy3
        let _value = fetcher.fetch_redirect_url(url.clone()).await?;
        let _value2 = fetcher.fetch_result(url).await?;
        todo!(
            "Only one value will be requested and processed based on url and maybe even options param"
        )
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn resolve_blocking<F>(&self, fetcher: &F) -> Result<ResolvedUrl, Box<dyn Error>>
    where
        F: BlockingUrlFetcher,
    {
        let url = Url::parse("https://localhost:8080")?; // https://tinyurl.com/2b5dwuy3
        let _value = fetcher.fetch_redirect_url(url.clone())?;
        let _value2 = fetcher.fetch_result(url)?;
        todo!(
            "Only one value will be requested and processed based on url and maybe even options param"
        )
    }
}
