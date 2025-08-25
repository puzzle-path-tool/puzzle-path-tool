use itertools::Itertools;
use linkme::distributed_slice;
use serde_json::Value;

pub mod examples;
pub mod exports;
pub mod variable;

#[distributed_slice]
static TYPES: [fn() -> String];

#[must_use]
pub fn load_types() -> String {
    TYPES
        .into_iter()
        .map(|f| format!("export {}", f()))
        .join("\n\n")
}

pub trait PuzzptApiExport {
    #[must_use]
    fn tag_value() -> &'static str;
    #[must_use]
    fn tag_name() -> &'static str {
        "export_tag"
    }
    #[must_use]
    fn has_tag(value: &Value) -> bool {
        value
            .get(Self::tag_name())
            .and_then(|v| v.as_str())
            .is_some_and(|v| v == Self::tag_value())
    }
}
