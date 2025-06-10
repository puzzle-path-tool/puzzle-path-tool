use url::Url;

pub struct PuzzleFormat {} //TODO

pub struct ResolvedUrl {
    inner: ResolvedUrlInner,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("Placeholder Error")]
    Placeholder,
}

pub(crate) enum ResolvedUrlInner {
    FPuzzles(Box<str>),
    SudokuPad(SudokuPadFullUrl),
    SudokuMaker(Box<str>),
    Penpa(Box<Url>),
}

pub(crate) enum SudokuPadFullUrl {
    Scl(Box<str>),
    Scf(Box<str>),
    FPuz(Box<str>),
}

impl ResolvedUrl {
    pub(crate) fn new(inner: ResolvedUrlInner) -> Self {
        Self { inner }
    }
}

impl ResolvedUrl {
    #[allow(clippy::missing_errors_doc)]
    pub fn decode(&self) -> Result<PuzzleFormat, DecodeError> {
        match &self.inner {
            ResolvedUrlInner::FPuzzles(id) => {
                todo!("{id}")
            }
            ResolvedUrlInner::SudokuPad(full_url) => match full_url {
                SudokuPadFullUrl::FPuz(id) => {
                    todo!("{id}")
                }
                SudokuPadFullUrl::Scl(id) => {
                    todo!("{id}")
                }
                SudokuPadFullUrl::Scf(id) => {
                    todo!("{id}")
                }
            },
            ResolvedUrlInner::SudokuMaker(id) => {
                todo!("{id}")
            }
            ResolvedUrlInner::Penpa(id) => {
                todo!("{id}")
            }
        }
    }
}
