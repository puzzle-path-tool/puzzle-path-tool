use crate::layers::second_layer::{
    self, StepId, TableId,
};

mod steps;

#[derive(Debug, Clone, Copy)]
pub struct DeductionTable {
    id: TableId,
    length: usize,
}
impl DeductionTable {
    fn from_second_layer(deduction_table: &second_layer::tables::DeductionTable) -> DeductionTable {
        DeductionTable {
            id: deduction_table.get_id(),
            length: deduction_table.get_length(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArrayTable {
    id: TableId,
    ref_id: TableId,
    length: usize,
}
impl ArrayTable {
    fn from_second_layer(array_table: &second_layer::tables::ArrayTable) -> ArrayTable {
        ArrayTable {
            id: array_table.get_id(),
            ref_id: array_table.get_ref_id(),
            length: array_table.get_length(),
        }
    }
}

#[derive(Debug)]
struct IdSupplier {
    current: usize,
    conversion: Vec<(usize, usize)>,
}
impl IdSupplier {
    fn new() -> IdSupplier {
        IdSupplier {
            current: 0,
            conversion: vec![],
        }
    }
    fn next(&mut self) -> usize {
        let result = self.current;
        self.current += 1;
        result
    }
    fn convert(&mut self, old_id: usize) -> usize {
        if let Some((_, new_id)) = self.conversion.iter().find(|item| item.0 == old_id) {
            *new_id
        } else {
            let new_id = self.next();
            self.conversion.push((old_id, new_id));
            new_id
        }
    }
}
