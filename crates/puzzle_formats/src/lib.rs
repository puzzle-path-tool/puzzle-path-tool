#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum Region {
    InRegion(i32),
    NoRegion,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GridCell {
    #[serde(rename = "value")]
    value: Option<i32>,

    #[serde(rename = "given")] // Default is false
    given: Option<bool>,

    #[serde(rename = "region")] // Treat null and undefined Differently
    region: Option<i32>,

    #[serde(rename = "c")] // Parse Color
    c: Option<String>,

    #[serde(rename = "centerPencilMarks")]
    center_pencil_marks: Option<Vec<i32>>,

    #[serde(rename = "cornerPencilMarks")]
    corner_pencil_marks: Option<Vec<i32>>,

    #[serde(rename = "highlight")] // Parse Color
    highlight: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FPuzzlesFormat {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "author")]
    author: String,

    #[serde(rename = "ruleset")]
    ruleset: String,

    #[serde(rename = "size")]
    size: i32,

    #[serde(rename = "highlightConflicts")]
    highlight_conflicts: bool,

    #[serde(rename = "grid")]
    grid: Vec<Vec<GridCell>>,

    #[serde(rename = "diagonal+")]
    diagonal_positive: Option<bool>,

    #[serde(rename = "diagonal-")]
    diagonal_negative: Option<bool>,

    #[serde(rename = "antiknight")]
    antiknight: Option<bool>,

    #[serde(rename = "antiking")]
    antiking: Option<bool>,

    #[serde(rename = "disjointgroups")]
    disjointgroups: Option<bool>,

    #[serde(rename = "nonconsecutive")]
    nonconsecutive: Option<bool>,

    #[serde(rename = "extraregion")]
    extraregion: Option<Value>,

    #[serde(rename = "odd")]
    odd: Option<Value>,

    #[serde(rename = "even")]
    even: Option<Value>,

    #[serde(rename = "thermometer")]
    thermometer: Option<Value>,

    #[serde(rename = "palindrome")]
    palindrome: Option<Value>,

    #[serde(rename = "killercage")]
    killercage: Option<Value>,

    #[serde(rename = "littlekillersum")]
    littlekillersum: Option<Value>,

    #[serde(rename = "sandwichsum")]
    sandwichsum: Option<Value>,

    #[serde(rename = "difference")]
    difference: Option<Value>,

    #[serde(rename = "negative")]
    negative: Option<Value>,

    #[serde(rename = "ratio")]
    ratio: Option<Value>,

    #[serde(rename = "clone")]
    clone: Option<Value>,

    #[serde(rename = "arrow")]
    arrow: Option<Value>,

    #[serde(rename = "betweenline")]
    betweenline: Option<Value>,

    #[serde(rename = "minimum")]
    minimum: Option<Value>,

    #[serde(rename = "maximum")]
    maximum: Option<Value>,

    #[serde(rename = "xv")]
    xv: Option<Value>,

    #[serde(rename = "quadruple")]
    quadruple: Option<Value>,

    #[serde(rename = "text")]
    text: Option<Value>,

    #[serde(rename = "circle")]
    circle: Option<Value>,

    #[serde(rename = "rectangle")]
    rectangle: Option<Value>,

    #[serde(rename = "line")]
    line: Option<Value>,

    #[serde(rename = "cage")]
    cage: Option<Value>,
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
