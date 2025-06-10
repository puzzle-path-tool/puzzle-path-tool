#![cfg(any(feature = "rusqlite", feature = "tokio-rusqlite"))]

use rusqlite::OptionalExtension;
use url::Url;

mod rusqlite_async;
mod rusqlite_blocking;

#[cfg(feature = "tokio-rusqlite")]
pub use rusqlite_async::{FetchError, RusqliteUrlFetcherCache};
#[cfg(feature = "rusqlite")]
pub use rusqlite_blocking::{BlockingFetchError, RusqliteBlockingUrlFetcherCache};

fn create_tables(connection: &rusqlite::Connection) -> rusqlite::Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS redirect_cache (
                request_url     TEXT NOT NULL PRIMARY KEY,
                redirect_url    TEXT
            )",
        (),
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS result_cache (
                request_url     TEXT NOT NULL PRIMARY KEY,
                response_value  TEXT NOT NULL
            )",
        (),
    )?;

    Ok(())
}

fn store_redirect(
    connection: &rusqlite::Connection,
    url: &url::Url,
    value: Option<&url::Url>,
) -> rusqlite::Result<()> {
    connection.execute(
        "INSERT OR REPLACE INTO redirect_cache (request_url, redirect_url) VALUES (?1, ?2)",
        (url.to_string(), value.map(url::Url::to_string)),
    )?;
    Ok(())
}

fn store_result(
    connection: &rusqlite::Connection,
    url: &url::Url,
    value: &str,
) -> rusqlite::Result<()> {
    connection.execute(
        "INSERT OR REPLACE INTO result_cache (request_url, response_value) VALUES (?1, ?2)",
        (url.to_string(), value.to_string()),
    )?;
    Ok(())
}

#[derive(Debug, Clone)]
enum FetchReturn<T> {
    Found(T),
    NotThere,
}

impl<T> FetchReturn<T> {
    fn from_option(option: Option<T>) -> Self {
        match option {
            Some(v) => Self::Found(v),
            None => Self::NotThere,
        }
    }

    fn into_option(self) -> Option<T> {
        match self {
            Self::Found(v) => Some(v),
            Self::NotThere => None,
        }
    }
}

fn fetch_redirect_url(
    connection: &rusqlite::Connection,
    url: &url::Url,
) -> rusqlite::Result<Result<FetchReturn<Option<url::Url>>, url::ParseError>> {
    let mut statement =
        connection.prepare("SELECT redirect_url FROM redirect_cache WHERE request_url=?1")?;

    let redirect_url: Option<Option<String>> = statement
        .query_row((url.to_string(),), |row| row.get(0))
        .optional()?;

    let fetch_return = match redirect_url {
        Some(Some(url_string)) => Url::parse(&url_string).map(|url| FetchReturn::Found(Some(url))),
        Some(None) => Ok(FetchReturn::Found(None)),
        None => Ok(FetchReturn::NotThere),
    };

    Ok(fetch_return)
}

fn fetch_result(
    connection: &rusqlite::Connection,
    url: &url::Url,
) -> rusqlite::Result<FetchReturn<Box<str>>> {
    let mut statement =
        connection.prepare("SELECT response_value FROM result_cache WHERE request_url=?1")?;

    let redirect_url: Option<String> = statement
        .query_row((url.to_string(),), |row| row.get(0))
        .optional()?;

    let fetch_return =
        FetchReturn::from_option(redirect_url.map(std::string::String::into_boxed_str));

    Ok(fetch_return)
}
