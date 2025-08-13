use crate::layers::second_layer::{
    self, EnumTableId as LookUpTableId, TableId, tables::TableBundle as SecondLayerTables,
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

#[derive(Debug, Clone)]
pub struct LookUpTable {
    id: LookUpTableId,
    look_up_table: Vec<(i32, i32)>,
}
impl LookUpTable {
    fn from_second_layer_enum_int_mapping(
        mapping: second_layer::tables::EnumIntMapping,
        enum_tables: &SecondLayerTables,
    ) -> LookUpTable {
        if let Some(enum_table) = enum_tables.get_enum_by_id(mapping.get_ref_id()) {
            LookUpTable {
                id: mapping.get_id(),
                look_up_table: mapping
                    .get_look_up_table()
                    .iter()
                    .map(|(enum_value, result_number)| {
                        if let Some(key_number) = enum_table.convert(enum_value) {
                            (key_number, *result_number)
                        } else {
                            panic!()
                        }
                    })
                    .collect(),
            }
        } else {
            panic!()
        }
    }
    fn from_second_layer_int_enum_mapping(
        mapping: second_layer::tables::IntEnumMapping,
        enum_tables: &SecondLayerTables,
    ) -> LookUpTable {
        if let Some(enum_table) = enum_tables.get_enum_by_id(mapping.get_ref_id()) {
            LookUpTable {
                id: mapping.get_id(),
                look_up_table: mapping
                    .get_look_up_table()
                    .iter()
                    .map(|(key_number, enum_value)| {
                        if let Some(result_number) = enum_table.convert(enum_value) {
                            (*key_number, result_number)
                        } else {
                            panic!()
                        }
                    })
                    .collect(),
            }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct IdSupplier {
    current: usize,
    conversion: Vec<(WrappedId, usize)>,
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
    fn convert(&mut self, wrapped_id: WrappedId) -> usize {
        if let Some((_, new_id)) = self
            .conversion
            .iter()
            .find(|(item_wrapped_id, _)| *item_wrapped_id == wrapped_id)
        {
            *new_id
        } else {
            let new_id = self.next();
            self.conversion.push((wrapped_id, new_id));
            new_id
        }
    }
    fn new_wrapped_id(&mut self) -> WrappedId {
        WrappedId { id: self.next() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WrappedId {
    id: usize,
}
