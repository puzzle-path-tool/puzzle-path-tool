use std::error::Error;

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
            return Err(SomeError::SomeError);
        }

        let Some(domain) = url.domain() else {
            return Err(SomeError::SomeError);
        };

        let Some(mut segments) = url.path_segments() else {
            return Err(SomeError::SomeError);
        };

        let mut _query_pairs = url.query_pairs();

        match domain {
            "sudokupad.app" | "alpha.sudokupad.app" | "beta.sudokupad.app" => {
                // If it has the query param `puzzleid`, read it
                // Ignore other params
                // else
                // => if the first segment is `sudoku`, read all other segments and join them with '/'
                // => else read all segments and join them with '/'

                // The resulting id follows the format
                // SHORTID  or  (fpuz|fpuzzles)FPUZZLESID  or  (scl|ctc)SUDOKUPADID  or  (scf)SCFID

                todo!()
            }
            "f-puzzles.com" | "www.f-puzzles.com" => {
                // No segments
                // Check only first Query Param
                // => id = ShortID
                // => load = LongID
                // Ignore further Params
                todo!()
            }
            "sudokumaker.app" => {
                // Ignore Segments
                // Ignore other Params
                // Read `puzzle` Param
                todo!()
            }
            "swaroopg92.github.io" => {
                // Segments exact ["penpa-edit"]
                // Accepts either query params or fragments formatted the same ["#", "?", "#?", "?#"]

                if segments.next() != Some("penpa-edit") {
                    return Err(SomeError::SomeError);
                }
                todo!()
            }
            _ => Err(SomeError::SomeError),
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
