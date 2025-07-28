use crate::layers::second_layer::{self, TableId};


pub struct DeductionTable {
    id: TableId,
    length: usize,
}
impl DeductionTable {
    fn from_second_layer(deduction_table: &second_layer::tables::DeductionTable) -> DeductionTable {
        DeductionTable { id: deduction_table.get_id(), length: deduction_table.get_length() }
    }
}

pub struct ArrayTable {
    id: TableId,
    ref_id: TableId,
    length: usize,
}
impl ArrayTable {
    fn from_second_layer(array_table: &second_layer::tables::ArrayTable) -> ArrayTable {
        ArrayTable { id: array_table.get_id(), ref_id: array_table.get_ref_id(), length: array_table.get_length() }
    }
}
