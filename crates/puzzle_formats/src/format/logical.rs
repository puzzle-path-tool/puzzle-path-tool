use serde_json::Value;

pub struct LogicalPuzzle {
    elements: Box<[Element]>,
}

pub struct Element {
    element_type: Box<str>,
    data: Value,
}
