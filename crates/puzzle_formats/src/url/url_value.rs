use std::borrow::Cow;

use itertools::Itertools;
use url::Url;

use crate::url::resolved_url::SudokuPadFullUrl;

use super::{
    IntoUrl,
    resolved_url::{ResolvedUrl, ResolvedUrlInner},
    unresolved_url::{UnresolvedUrl, UnresolvedUrlInner},
};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid Url Scheme: {0}")]
    InvalidScheme(Box<str>),
    #[error("No Domain could be extracted from Url: {0}")]
    NoDomain(Url),
    #[error("Unknown Page at Url: {0}")]
    UnknownPage(Url),
    #[error("No Id could be extracted from Url: {0}")]
    MissingId(Url),
    #[error("Error Parsing Url: {0}")]
    UrlParseError(#[source] url::ParseError),
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
    pub fn parse(url: impl IntoUrl) -> Result<Self, ParseError> {
        use ResolvedUrlInner as R;
        use UnresolvedUrlInner as U;

        let url = url.into_url().map_err(ParseError::UrlParseError)?;

        if !matches!(url.scheme(), "http" | "https") {
            return Err(ParseError::InvalidScheme(url.scheme().into()));
        }

        let Some(domain) = url.domain() else {
            return Err(ParseError::NoDomain(url.clone()));
        };

        let Some(mut segments) = url.path_segments() else {
            return Err(ParseError::NoDomain(url.clone()));
        };

        let mut query_pairs = url.query_pairs();

        match domain {
            "sudokupad.app" | "alpha.sudokupad.app" | "beta.sudokupad.app" => {
                use SudokuPadFullUrl as F;

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
                    return Ok(Self::resolved(R::SudokuPad(F::FPuz(puzzleid.into()))));
                }

                if let Some(puzzleid) = puzzleid
                    .strip_prefix("scl")
                    .or_else(|| puzzleid.strip_prefix("ctc"))
                {
                    return Ok(Self::resolved(R::SudokuPad(F::Scl(puzzleid.into()))));
                }

                if let Some(puzzleid) = puzzleid.strip_prefix("scf") {
                    return Ok(Self::resolved(R::SudokuPad(F::Scf(puzzleid.into()))));
                }

                Ok(Self::unresolved(U::SudokuPad(puzzleid.into())))
            }
            "f-puzzles.com" | "www.f-puzzles.com" => {
                if segments.next().is_some() {
                    return Err(ParseError::UnknownPage(url));
                }

                let Some((k, v)) = query_pairs.next() else {
                    return Err(ParseError::MissingId(url));
                };

                match k.as_ref() {
                    "id" => {
                        let puzzleid = v;
                        Ok(Self::unresolved(U::FPuzzles(puzzleid.into_owned().into())))
                    }
                    "load" => {
                        let puzzleid = v;
                        Ok(Self::resolved(R::FPuzzles(puzzleid.into_owned().into())))
                    }
                    _k => Err(ParseError::MissingId(url)),
                }
            }
            "sudokumaker.app" => {
                let puzzleid = query_pairs.find_map(|(k, v)| (k == "puzzle").then_some(v));

                let Some(puzzleid) = puzzleid else {
                    return Err(ParseError::MissingId(url));
                };

                Ok(Self::resolved(R::SudokuMaker(
                    puzzleid.into_owned().into_boxed_str(),
                )))
            }
            "swaroopg92.github.io" => {
                if !(segments.next() == Some("penpa-edit") && segments.next().is_none()) {
                    return Ok(Self::unresolved(U::Unknown(Box::new(url))));
                }

                Ok(Self::resolved(R::Penpa(Box::new(url))))
            }
            _ => Ok(Self::unresolved(U::Unknown(Box::new(url)))),
        }
    }
}
