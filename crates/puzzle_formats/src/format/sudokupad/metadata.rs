use crate::serialization::is_default;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "source", default)]
    source: Box<str>,

    #[serde(rename = "title", default)]
    title: Box<str>,

    #[serde(rename = "author", default)]
    author: Box<str>,

    #[serde(rename = "rules", default)]
    rules: Box<str>,

    #[serde(
        rename = "msgcorrect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    msgcorrect: Option<Box<str>>,

    #[serde(rename = "antiknight", default, skip_serializing_if = "is_default")]
    antiknight: bool,

    #[serde(rename = "antiking", default, skip_serializing_if = "is_default")]
    antiking: bool,
}
