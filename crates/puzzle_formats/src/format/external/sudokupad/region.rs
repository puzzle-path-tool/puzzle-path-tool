use serde::{Deserialize, Serialize};

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(transparent)]
pub struct Region {
    cells: Box<[Pos<i32>]>,
}
