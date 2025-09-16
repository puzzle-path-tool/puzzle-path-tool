use std::sync::atomic;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TableId {
    value: usize,
}

static COUNTER_TABLE: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl TableId {
    pub(crate) fn new() -> TableId {
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
    pub(crate) fn new() -> StepId {
        StepId {
            value: COUNTER_STEP.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PathString {
    path: Vec<String>,
}
impl PathString {
    pub(crate) fn new() -> PathString {
        PathString { path: vec![] }
    }
    pub(crate) fn push(&mut self, value: String) {
        self.path.push(value)
    }
    pub(crate) fn out_of(&self, path: &PathString) -> Option<PathString> {
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
    pub(crate) fn new() -> EnumTableId {
        EnumTableId {
            value: COUNTER_ENUM.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IdSupplier {
    current: usize,
    wrapped_conversion: Vec<(WrappedId, usize)>,
    i32_conversion: Vec<(i32, usize)>,
}
impl IdSupplier {
    pub(crate) fn new() -> IdSupplier {
        IdSupplier {
            current: 0,
            wrapped_conversion: vec![],
            i32_conversion: vec![],
        }
    }
    pub(crate) fn next(&mut self) -> usize {
        let result = self.current;
        self.current += 1;
        result
    }
    pub(crate) fn convert_wrapped(&mut self, wrapped_id: WrappedId) -> usize {
        if let Some((_, new_id)) = self
            .wrapped_conversion
            .iter()
            .find(|(item_wrapped_id, _)| *item_wrapped_id == wrapped_id)
        {
            *new_id
        } else {
            let new_id = self.next();
            self.wrapped_conversion.push((wrapped_id, new_id));
            new_id
        }
    }
    pub(crate) fn convert_i32(&mut self, i32_id: i32) -> usize {
        if let Some((_, new_id)) = self
            .i32_conversion
            .iter()
            .find(|(item_i32_id, _)| *item_i32_id == i32_id)
        {
            *new_id
        } else {
            let new_id = self.next();
            self.i32_conversion.push((i32_id, new_id));
            new_id
        }
    }
    pub(crate) fn new_wrapped_id(&mut self) -> WrappedId {
        WrappedId { id: self.next() }
    }
    pub(crate) fn close_and_get_size(self) -> usize {
        self.current
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WrappedId {
    id: usize,
}
