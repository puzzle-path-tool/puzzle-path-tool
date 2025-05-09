#![allow(dead_code)]

use core::fmt;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize, de};
use serde_json::Value;

pub enum Region {
    InRegion(i32),
    NoRegion,
}

pub struct CellPos {
    row: i8,
    column: i8,
}

impl fmt::Debug for CellPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<R{}C{}>", self.row, self.column)
    }
}

impl Serialize for CellPos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cell_string = format!("R{}C{}", self.row, self.column);
        serializer.serialize_str(&cell_string)
    }
}

ctreg::regex! { CellPosRegex = r"R(?<row>-?\d+)C(?<column>-?\d+)" }
static CELL_POS_RE: LazyLock<CellPosRegex> = LazyLock::new(CellPosRegex::new);

impl<'de> Deserialize<'de> for CellPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let cell_string: String = Deserialize::deserialize(deserializer)?;

        let captures = CELL_POS_RE.captures(&cell_string).ok_or_else(|| {
            de::Error::custom(format!(
                "Invalid Format for Cell String `{cell_string}`, expected format `R<row>C<col>`"
            ))
        })?;

        let row = captures.row.content.parse().map_err(|err| {
            de::Error::custom(format!(
                "Invalid Cell Row for Cell String `{cell_string}`: {err}"
            ))
        })?;

        let column = captures.column.content.parse().map_err(|err| {
            de::Error::custom(format!(
                "Invalid Cell Column for Cell String `{cell_string}`: {err}"
            ))
        })?;

        Ok(Self { row, column })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CellCollectionWithValue {
    #[serde(rename = "cells")]
    cells: Box<[CellPos]>,
    #[serde(rename = "value")]
    value: Option<Box<str>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SingleCell {
    #[serde(rename = "cell")]
    cell: CellPos,
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
    odd: Option<Box<[SingleCell]>>,

    #[serde(rename = "even")]
    even: Option<Box<[SingleCell]>>,

    #[serde(rename = "thermometer")]
    thermometer: Option<Value>,

    #[serde(rename = "palindrome")]
    palindrome: Option<Value>,

    #[serde(rename = "killercage")]
    killercage: Option<Box<[CellCollectionWithValue]>>,

    #[serde(rename = "littlekillersum")]
    littlekillersum: Option<Value>,

    #[serde(rename = "sandwichsum")]
    sandwichsum: Option<Value>,

    #[serde(rename = "difference")]
    difference: Option<Box<[CellCollectionWithValue]>>,

    #[serde(rename = "negative")]
    negative: Option<Value>,

    #[serde(rename = "ratio")]
    ratio: Option<Box<[CellCollectionWithValue]>>,

    #[serde(rename = "clone")]
    clone: Option<Value>,

    #[serde(rename = "arrow")]
    arrow: Option<Value>,

    #[serde(rename = "betweenline")]
    betweenline: Option<Value>,

    #[serde(rename = "minimum")]
    minimum: Option<Box<[SingleCell]>>,

    #[serde(rename = "maximum")]
    maximum: Option<Box<[SingleCell]>>,

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

pub enum FPuzzlesURL {
    // https://[www].f-puzzles.com/?load=FPUZZLESID
    Normal(String),

    // https://f-puzzles.com/?id=TINYURLID
    // get the redirect from https://tinyurl.com/TINYURLID OR the url itself (f-puzzles does not do any safety check, for the redirect)
    Shortened(String),
}

pub enum SudokuPadUrl {
    // https://sudokupad.app/ANYTHING?puzzleid=ANYSUDOKUPADID
    // https://sudokupad.app/sudoku/ANYSUDOKUPADID
    // https://sudokupad.app/ANYSUDOKUPADID

    // ANYSUDOKUPADID = SHORTID // (fpuz|fpuzzles)FPUZZLESID // (scl|ctc)SUDOKUPADID
    Scl(String),
    FPuz(String),

    // get the correct id as text response from https://sudokupad.app/api/puzzle/SHORTID
    Shortened(String),
}

impl SudokuPadUrl {
    fn from_full_id(full_id: &str) -> Self {
        if full_id.len() > 20 {
            SudokuPadUrl::Scl(String::new())
        } else {
            SudokuPadUrl::Shortened(String::new())
        }
    }
}

pub enum PuzzleUrl {
    SudokuPad(SudokuPadUrl),
    FPuzzles(FPuzzlesURL),
    // sudokumaker.app/?puzzle=SUDOKUMAKERID
    SudokuMakerURL(String),
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
