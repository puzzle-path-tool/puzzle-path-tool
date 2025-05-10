#![allow(dead_code)]

use core::fmt;
use std::{borrow::Cow, sync::LazyLock};

use serde::{Deserialize, Serialize, de, ser::SerializeSeq};
use serde_json::Value;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Region {
    InRegion(i32),
    NoRegion,
}

#[derive(Eq, PartialEq, Clone, Copy)]
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

ctreg::regex! { CellPosRegex = r"^R(?<row>-?\d+)C(?<column>-?\d+)$" }
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellCollectionWithValue {
    #[serde(rename = "cells")]
    cells: Box<[CellPos]>,
    #[serde(rename = "value")]
    value: Option<Box<str>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct SingleCell {
    #[serde(rename = "cell")]
    cell: CellPos,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct GridCell {
    #[serde(rename = "value")]
    value: Option<StrOrInt>,

    #[serde(rename = "given", default, skip_serializing_if = "is_default")]
    given: bool,

    #[serde(rename = "region")] // Treat null and undefined Differently
    region: Option<i32>,

    #[serde(rename = "c")] // Parse Color
    c: Option<Box<str>>,

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

    #[serde(rename = "highlight")] // Parse Color
    highlight: Option<Box<str>>,
    // candidates
    //
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    #[serde(rename = "DR")]
    DownRight,
    #[serde(rename = "DL")]
    DownLeft,
    #[serde(rename = "UR")]
    UpRight,
    #[serde(rename = "UL")]
    UpLeft,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum NegativeFlag {
    #[serde(rename = "ratio")]
    Ratio,
    #[serde(rename = "xv")]
    XV,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct Negative {
    ratio: bool,
    xv: bool,
}

impl Serialize for Negative {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = vec![];
        if self.ratio {
            flags.push(NegativeFlag::Ratio);
        }
        if self.xv {
            flags.push(NegativeFlag::XV);
        }

        let mut seq = serializer.serialize_seq(Some(flags.len()))?;
        for flag in flags {
            seq.serialize_element(&flag)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Negative {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let flags: Vec<NegativeFlag> = Vec::deserialize(deserializer)?;

        let mut negatives = Negative::default();

        for flag in flags {
            match flag {
                NegativeFlag::Ratio => negatives.ratio = true,
                NegativeFlag::XV => negatives.xv = true,
            }
        }

        Ok(negatives)
    }
}

/*

Tool Constraints:

lines
cell
cells
cloneCells
direction
value
values

Cosmetics:

lines
cell
cells
direction
value
baseC
outlineC
fontC
size
width
height
angle

Boolean Constraints

diagonal+
diagonal-
antiknight
antiking
disjointgroups
nonconsecutive

TODO:
Make Options into Defaults (in both directions, eg. if not present => Default, if Default => leave out)
Make Option List into Empty List (also both Directions)

*/

fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq<T>,
{
    *value == T::default()
}

fn is_empty<T>(value: &[T]) -> bool {
    value.is_empty()
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct FPuzzlesFormat {
    #[serde(rename = "title", default)]
    title: Box<str>,

    #[serde(rename = "author", default)]
    author: Box<str>,

    #[serde(rename = "ruleset", default)]
    ruleset: Box<str>,

    #[serde(rename = "size")]
    size: i32,

    #[serde(rename = "highlightConflicts", default)]
    highlight_conflicts: bool,

    #[serde(rename = "grid")]
    grid: Box<[Box<[GridCell]>]>,

    #[serde(rename = "diagonal+", default, skip_serializing_if = "is_default")]
    diagonal_positive: bool,

    #[serde(rename = "diagonal-", default, skip_serializing_if = "is_default")]
    diagonal_negative: bool,

    #[serde(rename = "antiknight", default, skip_serializing_if = "is_default")]
    antiknight: bool,

    #[serde(rename = "antiking", default, skip_serializing_if = "is_default")]
    antiking: bool,

    #[serde(rename = "disjointgroups", default, skip_serializing_if = "is_default")]
    disjointgroups: bool,

    #[serde(rename = "nonconsecutive", default, skip_serializing_if = "is_default")]
    nonconsecutive: bool,

    #[serde(rename = "extraregion")]
    extraregion: Option<Value>,

    #[serde(rename = "odd", default, skip_serializing_if = "is_empty")]
    odd: Box<[SingleCell]>,

    #[serde(rename = "even", default, skip_serializing_if = "is_empty")]
    even: Box<[SingleCell]>,

    #[serde(rename = "thermometer")]
    thermometer: Option<Value>,

    #[serde(rename = "palindrome")]
    palindrome: Option<Value>,

    #[serde(rename = "killercage", default, skip_serializing_if = "is_empty")]
    killercage: Box<[CellCollectionWithValue]>,

    #[serde(rename = "littlekillersum")]
    littlekillersum: Option<Value>,

    #[serde(rename = "sandwichsum")]
    sandwichsum: Option<Value>,

    #[serde(rename = "difference", default, skip_serializing_if = "is_empty")]
    difference: Box<[CellCollectionWithValue]>,

    #[serde(rename = "negative", default, skip_serializing_if = "is_default")]
    negative: Negative,

    #[serde(rename = "ratio", default, skip_serializing_if = "is_empty")]
    ratio: Box<[CellCollectionWithValue]>,

    #[serde(rename = "clone")]
    clone: Option<Value>,

    #[serde(rename = "arrow")]
    arrow: Option<Value>,

    #[serde(rename = "betweenline")]
    betweenline: Option<Value>,

    #[serde(rename = "minimum", default, skip_serializing_if = "is_empty")]
    minimum: Box<[SingleCell]>,

    #[serde(rename = "maximum", default, skip_serializing_if = "is_empty")]
    maximum: Box<[SingleCell]>,

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
