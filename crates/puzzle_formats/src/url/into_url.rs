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

#[cfg(test)]
mod test {
    use url::Url;

    use crate::url::IntoUrl;

    #[test]
    fn convert() -> anyhow::Result<()> {
        let url_str: &str = "https://google.com/";
        let url_string: String = url_str.to_string();
        let url = Url::parse(url_str)?;

        assert_eq!(url, url_str.into_url()?, "Converting &str to Url");
        assert_eq!(
            url,
            url_string.clone().into_url()?,
            "Converting String to Url"
        );
        assert_eq!(url, (&url_string).into_url()?, "Converting &String to Url");
        assert_eq!(url, url.clone().into_url()?, "Converting Url to Url");

        assert_eq!(
            url_str,
            IntoUrl::as_str(&url_str),
            "Converting &str to &str"
        );
        assert_eq!(
            url_str,
            IntoUrl::as_str(&url_string),
            "Converting String to &str"
        );
        assert_eq!(
            url_str,
            IntoUrl::as_str(&&url_string),
            "Converting &String to &str"
        );
        assert_eq!(url_str, IntoUrl::as_str(&url), "Converting Url to &str");

        Ok(())
    }
}
