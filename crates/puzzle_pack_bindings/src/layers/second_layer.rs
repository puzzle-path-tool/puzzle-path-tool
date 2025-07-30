use std::sync::atomic;

pub(super) mod tables;
pub(super) mod steps;

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
