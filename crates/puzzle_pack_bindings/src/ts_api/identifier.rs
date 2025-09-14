use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::TYPES;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "IdentifierPuzzptApi")]
pub struct Identifier {
    #[serde(rename = "pack")]
    pack: String,
    #[serde(rename = "module")]
    module: Vec<String>,
    #[serde(rename = "name")]
    name: String,
}

#[distributed_slice(TYPES)]
static INDENTIFIER_TYPE: fn() -> String = Identifier::decl;

impl Identifier {
    #[must_use]
    pub fn name(&self) -> String {
        format!("{}_{}_{}", self.pack, self.module.join("-"), self.name)
    }
}
