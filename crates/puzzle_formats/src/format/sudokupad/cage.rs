use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cage {
    // TODO: ({cells: ([x, y]) [], value: "45", unique: true, hidden: true, type: "rowcol" })
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
