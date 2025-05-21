use serde::{Deserialize, Serialize};

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
