use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Eq,
    PartialEq,
    Clone,
    Copy,
    IntoPrimitive,
    TryFromPrimitive,
    Default,
)]
#[repr(u8)]
pub enum HighlightColor {
    #[default]
    #[serde(rename = "#FFFFFF")]
    White = 0,
    #[serde(rename = "#A8A8A8")]
    Gray = 1,
    #[serde(rename = "#000000")]
    Black = 2,
    #[serde(rename = "#FFA0A0")]
    Red = 3,
    #[serde(rename = "#FFE060")]
    Yellow = 4,
    #[serde(rename = "#FFFFB0")]
    LightYellow = 5,
    #[serde(rename = "#B0FFB0")]
    LightGreen = 6,
    #[serde(rename = "#60D060")]
    Green = 7,
    #[serde(rename = "#D0D0FF")]
    LightBlue = 8,
    #[serde(rename = "#8080F0")]
    Bleue = 9,
    #[serde(rename = "#FF80FF")]
    Lavender = 10,
    #[serde(rename = "#FFD0D0")]
    LightRed = 11,
}
