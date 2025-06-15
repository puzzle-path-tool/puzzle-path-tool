use itertools::Itertools;
use linkme::distributed_slice;

pub mod examples;

#[distributed_slice]
static TYPES: [fn() -> String];

#[must_use]
pub fn load_types() -> String {
    TYPES
        .into_iter()
        .map(|f| format!("export {}", f()))
        .join("\n\n")
}
