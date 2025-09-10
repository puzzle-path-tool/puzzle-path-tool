pub mod steps;

pub mod tables {
    use serde::de::value;

    use crate::layers::{
        id_helpers::{EnumTableId as LookUpTableId, TableId as ArrayId, TableId as DeductionId},
        second_layer::tables::{
            ArrayTable as SecondLayerArray, DeductionTable as SecondLayerDeduction,
            //EnumIntMapping as SecondLayerEnumToInt, IntEnumMapping as SecondLayerIntToEnum,
            TableBundle as SecondLayerTables,
        },
    };

    #[derive(Debug, Clone)]
    pub(crate) struct TableBundle {
        deduction_tables: Vec<DeductionTable>,
        array_tables: Vec<ArrayTable>,

        //lookup_tables: Vec<LookUpTable>,
    }
    impl TableBundle {
        pub(crate) fn from_second_layer(tables: &SecondLayerTables) -> TableBundle {
            let deduction_tables = tables
                .get_deduction_tables()
                .iter()
                .map(|deduction_table| DeductionTable::from_second_layer(deduction_table))
                .collect();
            let array_tables = tables
                .get_array_tables()
                .iter()
                .map(|array_table| ArrayTable::from_second_layer(array_table))
                .collect();
            /*let mut lookup_tables: Vec<LookUpTable> = tables
                .get_enum_mappings()
                .0
                .iter()
                .map(|table| LookUpTable::from_second_layer_enum_int_mapping(table, tables))
                .collect();
            lookup_tables.append(
                &mut tables
                    .get_enum_mappings()
                    .1
                    .iter()
                    .map(|table| LookUpTable::from_second_layer_int_enum_mapping(table, tables))
                    .collect(),
            );*/
            TableBundle {
                deduction_tables,
                array_tables,
                //lookup_tables,
            }
        }
    }

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

    /* 
    #[derive(Debug, Clone)]
    pub struct LookUpTable {
        pub(crate) id: LookUpTableId,
        pub(crate) look_up_table: Vec<(i32, i32)>,
    }

    impl LookUpTable {
        pub(crate) fn from_second_layer_enum_int_mapping(
            mapping: &SecondLayerEnumToInt,
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
            mapping: &SecondLayerIntToEnum,
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
        pub fn convert(&self, value: i32) -> Option<i32> {
            self.look_up_table
                .iter()
                .find(|(key, _)| *key == value)
                .and_then(|(_, converted)| Some(*converted))
        }
    }*/
}

