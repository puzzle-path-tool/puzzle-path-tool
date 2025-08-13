use std::sync::atomic;

pub(super) mod steps;
pub(super) mod tables;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TableId {
    value: usize,
}

static COUNTER_TABLE: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl TableId {
    fn new() -> TableId {
        TableId {
            value: COUNTER_TABLE.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

static COUNTER_STEP: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StepId {
    value: usize,
}

impl StepId {
    fn new() -> TableId {
        TableId {
            value: COUNTER_STEP.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PathString {
    path: Vec<String>,
}
impl PathString {
    fn new() -> PathString {
        PathString { path: vec![] }
    }
    fn push(&mut self, value: String) {
        self.path.push(value)
    }
    fn out_of(&self, path: &PathString) -> Option<PathString> {
        if self.path.len() > path.path.len() {
            let mut self_path = self.path.clone();
            let self_rest = self_path.split_off(path.path.len());
            if self_path == path.path {
                Some(PathString { path: self_rest })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum FieldId {
    Primitive(usize),
    Array(TableId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnumTableId {
    value: usize,
}

static COUNTER_ENUM: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl EnumTableId {
    fn new() -> EnumTableId {
        EnumTableId {
            value: COUNTER_ENUM.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}
