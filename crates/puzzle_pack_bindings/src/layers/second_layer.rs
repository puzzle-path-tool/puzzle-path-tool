use std::sync::atomic;

pub(super) mod tables;
pub(super) mod steps;

#[derive(Debug, Clone, Copy)]
pub struct TableId {
    value: usize,
}

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl TableId {
    fn new() -> TableId {
        TableId {
            value: COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}
