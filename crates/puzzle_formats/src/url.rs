use serde_json::Value;

mod resolved_url;
mod unresolved_url;
pub mod url_fetcher;
mod url_value;

pub use resolved_url::*;
pub use unresolved_url::*;
pub use url_value::*;

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
