use serde::{Deserialize, Serialize};

use crate::serialization::{StrOrInt, deserialize_some, is_default, is_empty};

use super::{highlight_color::HighlightColor, region::Region};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct GridCell {
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    value: Option<StrOrInt>,

    #[serde(rename = "given", default, skip_serializing_if = "is_default")]
    given: bool,

    // null: None
    // undefined: Choose Default, based on position (get this as parameter somehow?, maybe add post processing step?)
    #[serde(
        rename = "region",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_some"
    )]
    region: Option<Region>,

    #[serde(rename = "c", default, skip_serializing_if = "is_default")]
    c: HighlightColor,

    #[serde(
        rename = "centerPencilMarks",
        default,
        skip_serializing_if = "is_empty"
    )]
    center_pencil_marks: Box<[StrOrInt]>,

    #[serde(
        rename = "cornerPencilMarks",
        default,
        skip_serializing_if = "is_empty"
    )]
    corner_pencil_marks: Box<[StrOrInt]>,

    #[serde(rename = "highlight", default, skip_serializing_if = "is_default")]
    highlight: HighlightColor,

    #[serde(rename = "candidates", default, skip_serializing_if = "is_empty")]
    candidates: Box<[i32]>,
}
