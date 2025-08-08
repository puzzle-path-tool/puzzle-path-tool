use itertools::Itertools;

use crate::layers::second_layer::{FieldId, PathString, TableId, steps::ObjectOutput};

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
            .map(|(name, field_type)| {
                Field::new(
                    name,
                    PathString::new(),
                    id,
                    &mut field_id_supplier,
                    field_type,
                )
            })
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
    pub(crate) fn get_field_id_by_name(&self, name: &PathString) -> Option<FieldId> {
        self.fields.iter().find_map(|field_item| {
            let sub_fields = field_item.flatten();
            sub_fields.iter().find_map(
                |(id, field_name)| {
                    if name == field_name { Some(*id) } else { None }
                },
            )
        })
    }
    pub(crate) fn table_fields(
        &self,
        array_tables: &Vec<&ArrayTable>,
        partial: &Option<PathString>,
    ) -> Vec<(FieldId, PathString)> {
        let mut fields = self.fields.iter().map(|item| item.flatten()).concat();
        fields.append(
            &mut array_tables
                .iter()
                .filter_map(|array_table| {
                    if array_table.get_ref_id() == self.get_id() {
                        Some((FieldId::Array(array_table.get_id()), array_table.get_name()))
                    } else {
                        None
                    }
                })
                .collect(),
        );
        if let Some(partial) = partial {
            fields
                .iter()
                .filter_map(|(field_id, name)| {
                    if let Some(new_name) = name.out_of(partial) {
                        Some((*field_id, new_name))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            fields
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArrayTable {
    id: super::TableId,
    ref_id: super::TableId,
    ref_name: PathString,
    field: Field,
    length: usize,
}
impl ArrayTable {
    fn new(
        name: &String,
        ref_name: PathString,
        ref_id: super::TableId,
        field_type: &TODO_FlatFieldTypeStandIn,
    ) -> (TableId, Vec<ArrayTable>) {
        let id = super::TableId::new();
        let mut field_id_supplier = FieldIdSupplier::new();
        let (field, mut array_tables) = Field::new(
            name,
            PathString::new(),
            id,
            &mut field_id_supplier,
            &TODO_FieldTypeStandIn::Flat(field_type.clone()),
        );
        let array_table = ArrayTable {
            id,
            ref_id,
            ref_name,
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
    pub(crate) fn get_name(&self) -> PathString {
        let mut name = self.ref_name.clone();
        name.push(self.field.get_name().to_string());
        name
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
        name: String,
        id: TableId,
    },
}
impl Field {
    fn new(
        name: &String,
        mut ref_name: PathString,
        ref_id: super::TableId,
        id_supplier: &mut FieldIdSupplier,
        field_type: &TODO_FieldTypeStandIn,
    ) -> (Field, Vec<ArrayTable>) {
        match field_type {
            TODO_FieldTypeStandIn::Array(flat_field_type) => {
                let (array_id, array_tables) =
                    ArrayTable::new(name, ref_name, ref_id, flat_field_type);
                (
                    Field::Array {
                        id: array_id,
                        name: name.clone(),
                    },
                    array_tables,
                )
            }
            TODO_FieldTypeStandIn::Flat(flat_field_type) => {
                match flat_field_type {
                    TODO_FlatFieldTypeStandIn::Object(items) => {
                        ref_name.push(name.to_string());
                        let (fields, array_tables) = items
                        .iter()
                        .map(|(field_name, field_type)| {
                            Field::new(field_name, ref_name.clone(), ref_id, id_supplier, field_type)
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
                }
            }
        }
    }
    pub(crate) fn flatten(&self) -> Vec<(FieldId, PathString)> {
        self.build_flat_type(PathString::new())
    }
    fn build_flat_type(&self, path: PathString) -> Vec<(FieldId, PathString)> {
        match self {
            Field::Primitive {
                id,
                name,
                field_type: _,
            } => vec![(FieldId::Primitive(*id), {
                let mut path = path;
                path.push(name.to_string());
                path
            })],
            Field::Object { name, fields } => {
                let mut path = path.clone();
                path.push(name.to_string());
                fields
                    .iter()
                    .map(|item| item.build_flat_type(path.clone()))
                    .concat()
            }
            Field::Array { id, name } => vec![(FieldId::Array(*id), {
                let mut path = path;
                path.push(name.to_string());
                path
            })],
        }
    }
    fn get_name(&self) -> &String {
        match self {
            Field::Primitive {
                id: _,
                name,
                field_type: _,
            }
            | Field::Object { name, fields: _ }
            | Field::Array { name, id: _ } => name,
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
