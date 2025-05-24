use url::{ParseError, Url};

pub trait IntoUrl {
    /// Parse the type into a Url, if necessary.
    ///
    /// # Errors
    ///
    /// See [`Url::parse`](::url::Url::parse) for more details.
    fn into_url(self) -> Result<Url, ParseError>;

    /// Return the serialization of this URL.
    fn as_str(&self) -> &str;
}

impl IntoUrl for Url {
    fn into_url(self) -> Result<Url, ParseError> {
        Ok(self)
    }

    fn as_str(&self) -> &str {
        Url::as_str(self)
    }
}

impl IntoUrl for String {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(String::as_str(&self))
    }

    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}

impl IntoUrl for &str {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(self)
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl IntoUrl for &String {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(String::as_str(self))
    }

    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}
