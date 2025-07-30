use itertools::Itertools;

use crate::layers::second_layer::TableId;

#[derive(Debug, Clone, Copy)]
struct FieldIdSupplier {
    current: usize,
}
impl FieldIdSupplier {
    fn new() -> FieldIdSupplier {
        FieldIdSupplier { current: 0 }
    }
    fn next(&mut self) -> usize {
        let result = self.current;
        self.current += 1;
        result
    }
    fn close_and_get_size(self) -> usize {
        self.current
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DeductionTable {
    id: super::TableId,
    name: String,
    description: String,
    fields: Vec<Field>,
    length: usize,
}
impl DeductionTable {
    fn new(
        name: String,
        field_types: Vec<(String, TODO_FieldTypeStandIn)>,
        description: String,
    ) -> (DeductionTable, Vec<ArrayTable>) {
        let id = super::TableId::new();
        let mut field_id_supplier = FieldIdSupplier::new();
        let (fields, array_tables) = field_types
            .iter()
            .map(|(name, field_type)| Field::new(name, id, &mut field_id_supplier, field_type))
            .fold(
                (vec![], vec![]),
                |(mut field_acc, mut array_acc), (field_item, mut array_tables_item)| {
                    field_acc.push(field_item);
                    array_acc.append(&mut array_tables_item);
                    (field_acc, array_acc)
                },
            );
        (
            DeductionTable {
                id,
                name,
                description,
                fields,
                length: field_id_supplier.close_and_get_size(),
            },
            array_tables,
        )
    }
    pub(crate) fn get_id(&self) -> super::TableId {
        self.id
    }
    pub(crate) fn get_length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayTable {
    id: super::TableId,
    ref_id: super::TableId,
    field: Field,
    length: usize,
}
impl ArrayTable {
    fn new(
        name: &String,
        ref_id: super::TableId,
        field_type: &TODO_FlatFieldTypeStandIn,
    ) -> (TableId, Vec<ArrayTable>) {
        let id = super::TableId::new();
        let mut field_id_supplier = FieldIdSupplier::new();
        let (field, mut array_tables) = Field::new(
            name,
            id,
            &mut field_id_supplier,
            &TODO_FieldTypeStandIn::Flat(field_type.clone()),
        );
        let array_table = ArrayTable {
            id,
            ref_id,
            field,
            length: field_id_supplier.close_and_get_size(),
        };
        let table_id = array_table.get_id();
        array_tables.push(array_table);
        (table_id, array_tables)
    }
    pub(crate) fn get_id(&self) -> super::TableId {
        self.id
    }
    pub(crate) fn get_ref_id(&self) -> super::TableId {
        self.ref_id
    }
    pub(crate) fn get_length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Field {
    Primitive {
        id: usize,
        name: String,
        field_type: FieldType,
    },
    Object {
        name: String,
        fields: Vec<Field>,
    },
    Array {
        id: TableId,
    },
}
impl Field {
    fn new(
        name: &String,
        ref_id: super::TableId,
        id_supplier: &mut FieldIdSupplier,
        field_type: &TODO_FieldTypeStandIn,
    ) -> (Field, Vec<ArrayTable>) {
        match field_type {
            TODO_FieldTypeStandIn::Array(flat_field_type) => {
                let (array_id, array_tables) = ArrayTable::new(name, ref_id, flat_field_type);
                (Field::Array { id: array_id }, array_tables)
            }
            TODO_FieldTypeStandIn::Flat(flat_field_type) => match flat_field_type {
                TODO_FlatFieldTypeStandIn::Object(items) => {
                    let (fields, array_tables) = items
                        .iter()
                        .map(|(field_name, field_type)| {
                            Field::new(field_name, ref_id, id_supplier, field_type)
                        })
                        .fold(
                            (vec![], vec![]),
                            |(mut field_acc, mut array_acc),
                             (field_item, mut array_tables_item)| {
                                field_acc.push(field_item);
                                array_acc.append(&mut array_tables_item);
                                (field_acc, array_acc)
                            },
                        );
                    (
                        Field::Object {
                            name: name.clone(),
                            fields,
                        },
                        array_tables,
                    )
                }
                TODO_FlatFieldTypeStandIn::Primitive(field_type) => (
                    Field::Primitive {
                        id: id_supplier.next(),
                        name: name.clone(),
                        field_type: field_type.clone(),
                    },
                    vec![],
                ),
            },
        }
    }
    pub(crate) fn flatten(&self) -> (Vec<usize>, Vec<TableId>) {
        match self {
            Field::Primitive { id, name: _, field_type: _ } => {
                (vec![*id], vec![])
            },
            Field::Object { name: _, fields } => {
                fields.iter().fold((vec![], vec![]), |(mut id_acc, mut array_acc), item|{
                    let (mut current_ids, mut current_arrays) = item.flatten();
                    id_acc.append(&mut current_ids);
                    array_acc.append(&mut current_arrays);
                    (id_acc, array_acc)
                })
            },
            Field::Array { id } => {
                (vec![], vec![*id])
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum FieldType {
    Number,
    Boolean,
    Enum(Vec<String>),
}
impl FieldType {}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TODO_FieldTypeStandIn {
    Array(TODO_FlatFieldTypeStandIn),
    Flat(TODO_FlatFieldTypeStandIn),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TODO_FlatFieldTypeStandIn {
    Object(Vec<(String, TODO_FieldTypeStandIn)>),
    Primitive(FieldType),
}
