mod steps;

mod tables {
    use crate::layers::{
        id_helpers::{EnumTableId as LookUpTableId, TableId as ArrayId, TableId as DeductionId},
        second_layer::tables::{
            ArrayTable as SecondLayerArray, DeductionTable as SecondLayerDeduction,
            EnumIntMapping as SecondLayerEnumToInt, IntEnumMapping as SecondLayerIntToEnum,
            TableBundle as SecondLayerTables,
        },
    };

    #[derive(Debug, Clone, Copy)]
    pub struct DeductionTable {
        pub(crate) id: DeductionId,
        pub(crate) length: usize,
    }

    impl DeductionTable {
        pub(crate) fn from_second_layer(deduction_table: &SecondLayerDeduction) -> DeductionTable {
            DeductionTable {
                id: deduction_table.get_id(),
                length: deduction_table.get_length(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ArrayTable {
        pub(crate) id: ArrayId,
        pub(crate) ref_id: DeductionId,
        pub(crate) length: usize,
    }

    impl ArrayTable {
        pub(crate) fn from_second_layer(array_table: &SecondLayerArray) -> ArrayTable {
            ArrayTable {
                id: array_table.get_id(),
                ref_id: array_table.get_ref_id(),
                length: array_table.get_length(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct LookUpTable {
        pub(crate) id: LookUpTableId,
        pub(crate) look_up_table: Vec<(i32, i32)>,
    }

    impl LookUpTable {
        pub(crate) fn from_second_layer_enum_int_mapping(
            mapping: SecondLayerEnumToInt,
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
        pub(crate) fn from_second_layer_int_enum_mapping(
            mapping: SecondLayerIntToEnum,
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
}
