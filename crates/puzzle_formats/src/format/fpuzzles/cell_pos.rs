use core::fmt;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize, de};

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
