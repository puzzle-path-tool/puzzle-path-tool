use serde_json::Value;

pub struct LogicalPuzzle {
    elements: Box<[Element]>,
}

pub struct Element {
    element_type: Box<str>, // "core:german_whisper"
    data: Value,            // {cells: [R1C2, R2C3, ...]}
}
