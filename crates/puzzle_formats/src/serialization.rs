use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize};

pub fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq<T>,
{
    *value == T::default()
}

pub fn is_empty<T>(value: &[T]) -> bool {
    value.is_empty()
}

/// Deserialize missing fields and explicit nulls differently.
///
/// If a value is present, this returns Some, the resolution of null is up to the contained type.
///
/// If no value is present, this returns None.
///
///
///
/// For Serialization, use `#[serde(skip_serializing_if = "Option::is_none")]`
///
/// # Errors
///
/// This function will return an error, only if the contained type's serialization fails.
///
/// # Examples
///
/// ```
/// use serde::{Deserialize};
/// use puzzle_formats::serialization::deserialize_some;
///
/// #[derive(Debug, Deserialize, Eq, PartialEq)]
/// struct Data {
///     #[serde(default, deserialize_with = "deserialize_some")]
///     value_one: Option<Option<i32>>,
///     #[serde(default, deserialize_with = "deserialize_some")]
///     value_null: Option<Option<i32>>,
///     #[serde(default, deserialize_with = "deserialize_some")]
///     value_undefined: Option<Option<i32>>,
/// }
///
/// // Option<i32> can instead be any type, with a serializer that can serialize null
///
/// let json = r#"{ "value_one": 1, "value_null": null }"#;
/// let data: Data = serde_json::from_str(json).unwrap();
///
/// assert_eq!(data.value_one, Some(Some(1)));
/// assert_eq!(data.value_null, Some(None));
/// assert_eq!(data.value_undefined, None);
/// ```
pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum StrOrInt {
    Str(Box<str>),
    Int(i32),
}

impl StrOrInt {
    #[must_use]
    pub fn as_str(&self) -> Cow<str> {
        match self {
            StrOrInt::Str(str) => Cow::Borrowed(str),
            StrOrInt::Int(n) => Cow::Owned(n.to_string()),
        }
    }

    #[must_use]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            StrOrInt::Str(str) => str.parse().ok(),
            StrOrInt::Int(n) => Some(*n),
        }
    }
}

impl From<String> for StrOrInt {
    fn from(value: String) -> Self {
        StrOrInt::Str(value.into_boxed_str())
    }
}

impl From<&str> for StrOrInt {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<i32> for StrOrInt {
    fn from(value: i32) -> Self {
        StrOrInt::Int(value)
    }
}
