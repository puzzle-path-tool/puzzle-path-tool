use itertools::Itertools;


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
pub(super) struct DeductionTable {
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
        let (fields, mut array_tables) = field_types
            .iter()
            .filter_map(|(name, field_type)| match field_type {
                TODO_FieldTypeStandIn::Array(_flat_field_type) => None,
                TODO_FieldTypeStandIn::Flat(flat_field_tiype) => Some(Field::new(
                    name,
                    id,
                    &mut field_id_supplier,
                    flat_field_tiype,
                )),
            })
            .fold(
                (vec![], vec![]),
                |(mut field_acc, mut array_acc), (field_item, mut array_tables_item)| {
                    field_acc.push(field_item);
                    array_acc.append(&mut array_tables_item);
                    (field_acc, array_acc)
                },
            );
        array_tables.append(
            &mut field_types
                .iter()
                .filter_map(|(field_name, field_type)| match field_type {
                    TODO_FieldTypeStandIn::Array(field_type) => {
                        Some(ArrayTable::new(field_name, id, field_type))
                    }
                    TODO_FieldTypeStandIn::Flat(_todo_flat_field_type_stand_in) => None,
                })
                .concat(),
        );
        (DeductionTable { id, name, description, fields, length: field_id_supplier.close_and_get_size() }, array_tables)
    }
    pub(super) fn get_id(&self) -> super::TableId {
        self.id
    }
    pub(super) fn get_length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone)]
pub(super) struct ArrayTable {
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
    ) -> Vec<ArrayTable> {
        let id = super::TableId::new();
        let mut field_id_supplier = FieldIdSupplier::new();
        let (field, mut array_tables) =
            Field::new(name, id, &mut field_id_supplier, field_type);
        array_tables.push(ArrayTable { id, ref_id, field, length: field_id_supplier.close_and_get_size() });
        array_tables
    }
    pub(super) fn get_id(&self) -> super::TableId {
        self.id
    }
    pub(super) fn get_ref_id(&self) -> super::TableId {
        self.ref_id
    }
    pub(super) fn get_length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone)]
enum Field {
    Primitive {
        id: usize,
        name: String,
        field_type: FieldType,
    },
    Object {
        name: String,
        fields: Vec<Field>,
    },
}
impl Field {
    fn new(
        name: &String,
        ref_id: super::TableId,
        id_supplier: &mut FieldIdSupplier,
        field_type: &TODO_FlatFieldTypeStandIn,
    ) -> (Field, Vec<ArrayTable>) {
        match field_type {
            TODO_FlatFieldTypeStandIn::Object(items) => {
                let (fields, mut array_tables) = items
                    .iter()
                    .filter_map(|(field_name, field_type)| match field_type {
                        TODO_FieldTypeStandIn::Array(_field_type) => None,
                        TODO_FieldTypeStandIn::Flat(todo_flat_field_type_stand_in) => {
                            Some(Field::new(
                                field_name,
                                ref_id,
                                id_supplier,
                                todo_flat_field_type_stand_in,
                            ))
                        }
                    })
                    .fold(
                        (vec![], vec![]),
                        |(mut field_acc, mut array_acc), (field_item, mut array_tables_item)| {
                            field_acc.push(field_item);
                            array_acc.append(&mut array_tables_item);
                            (field_acc, array_acc)
                        },
                    );
                array_tables.append(
                    &mut items
                        .iter()
                        .filter_map(|(field_name, field_type)| match field_type {
                            TODO_FieldTypeStandIn::Array(field_type) => {
                                Some(ArrayTable::new(field_name, ref_id, field_type))
                            }
                            TODO_FieldTypeStandIn::Flat(_todo_flat_field_type_stand_in) => None,
                        })
                        .concat(),
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
