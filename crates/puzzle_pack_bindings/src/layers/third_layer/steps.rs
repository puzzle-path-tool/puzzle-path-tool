use crate::layers::{
    second_layer::{self, FieldId, PathString, StepId, TableId},
    third_layer::IdSupplier,
};

pub struct LogicStep {
    id: StepId,
    match_statement: ValueOperation,
    step_objects: Vec<StepObject>,
    step_sets: Vec<SetObject>,
}
impl LogicStep {
    fn from_second_layer(
        logic_step: &second_layer::steps::LogicStep,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> LogicStep {
        LogicStep {
            id: logic_step.get_id(),
            match_statement: ValueOperation::from_second_layer_bool_output(
                logic_step.get_match_statement(),
                logic_step,
                &mut IdSupplier::new(),
                tables,
            ),
            step_objects: logic_step
                .get_step_objects()
                .iter()
                .map(|&x| StepObject::from_second_layer(x))
                .collect(),
            step_sets: logic_step
                .get_step_sets()
                .iter()
                .map(|&x| SetObject::from_second_layer(x))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
enum StepObject {
    DeductionObject {
        id: usize,
        table: TableId,
        in_pool: bool,
        emmit_or_consum: bool,
    },
    BuildObject {
        id: usize,
        fields: BuildObjectFields,
    },
}
impl StepObject {
    fn from_second_layer(object: &second_layer::steps::StepObject) -> StepObject {
        match object {
            second_layer::steps::StepObject::DeductionObject {
                id,
                table,
                in_pool,
                emmit_or_consum,
            } => StepObject::DeductionObject {
                id: *id,
                table: *table,
                in_pool: *in_pool,
                emmit_or_consum: *emmit_or_consum,
            },
            second_layer::steps::StepObject::BuildObject { id, fields } => {
                StepObject::BuildObject {
                    id: *id,
                    fields: BuildObjectFields::from_second_layer(fields),
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
struct BuildObjectFields {
    value_fields: Vec<(FieldId, PathString)>,
    array_fields: Vec<(TableId, Vec<(FieldId, PathString)>)>,
}
impl BuildObjectFields {
    fn from_second_layer(fields: &second_layer::steps::BuildObjectFields) -> BuildObjectFields {
        let (value_fields, array_fields) = fields.flat_type();
        BuildObjectFields {
            value_fields,
            array_fields,
        }
    }
}
#[derive(Debug, Clone)]
enum SetObject {
    SetOfObjects(StepObject),
    SetOfSets(Box<SetObject>),
}
impl SetObject {
    fn from_second_layer(set: &second_layer::steps::SetObject) -> SetObject {
        match set {
            second_layer::steps::SetObject::SetOfObjects(step_object) => {
                Self::SetOfObjects(StepObject::from_second_layer(step_object))
            }
            second_layer::steps::SetObject::SetOfSets(set_object) => {
                Self::SetOfSets(Box::new(SetObject::from_second_layer(set_object.as_ref())))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ValueOperation {
    TwoValueOp {
        first: Box<ValueOperation>,
        second: Box<ValueOperation>,
        operator: TwoValueOperator,
    },
    TwoSetOp {
        first: Box<SetOperation>,
        second: Box<SetOperation>,
        operator: ValueTwoSetOperator,
    },
    //ElementOfSet as filtered setsize > 0
    SetSize {
        set: Box<SetOperation>,
    },
    FixedValue {
        value: i32,
    },
    FieldValue {
        object_id: usize,
        field_id: usize,
    },
    MappingStandIn {
        mapping_id: usize,
    },
    MappingStandInFieldValue {
        mapping_id: usize,
        field_id: usize,
    },
    Mapping, //ToDo
}
impl ValueOperation {
    fn from_second_layer_bool_output(
        bool_output: &second_layer::steps::BooleanOutput,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match bool_output {
            second_layer::steps::BooleanOutput::BoolCombination {
                first,
                second,
                operator,
            } => Self::TwoValueOp {
                first: Box::new(ValueOperation::from_second_layer_bool_output(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(ValueOperation::from_second_layer_bool_output(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    second_layer::steps::BoolCombinator::And => TwoValueOperator::Add,
                    second_layer::steps::BoolCombinator::Or => TwoValueOperator::Or,
                    second_layer::steps::BoolCombinator::XOr => TwoValueOperator::XOr,
                },
            },
            second_layer::steps::BooleanOutput::SetComparison {
                first,
                second,
                operator,
            } => Self::TwoSetOp {
                first: Box::new(SetOperation::from_second_layer(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(SetOperation::from_second_layer(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    second_layer::steps::SetComparor::Equal => ValueTwoSetOperator::Equal,
                    second_layer::steps::SetComparor::SubsetOf => ValueTwoSetOperator::SubsetOf,
                    second_layer::steps::SetComparor::TrueSubsetOf => {
                        ValueTwoSetOperator::TrueSubsetOf
                    }
                    second_layer::steps::SetComparor::SupersetOf => ValueTwoSetOperator::SupersetOf,
                    second_layer::steps::SetComparor::TrueSupersetOf => {
                        ValueTwoSetOperator::TrueSupersetOf
                    }
                },
            },
            second_layer::steps::BooleanOutput::ElementOfSet { element, set } => {
                let stand_in_id = map_id_supplier.next();
                let mapping = match element.as_ref() {
                    second_layer::steps::Output::Object(output) => {
                        ValueOperation::object_element_of_set(
                            output,
                            set,
                            stand_in_id,
                            step,
                            map_id_supplier,
                            tables,
                        )
                    }
                    second_layer::steps::Output::Set(output) => ValueOperation::TwoSetOp {
                        first: Box::new(SetOperation::MappingStandIn {
                            mapping_id: stand_in_id,
                        }),
                        second: Box::new(SetOperation::from_second_layer(
                            output,
                            step,
                            map_id_supplier,
                            tables,
                        )),
                        operator: ValueTwoSetOperator::Equal,
                    },
                    second_layer::steps::Output::Primitive(primitive_output) => {
                        ValueOperation::TwoValueOp {
                            first: Box::new(ValueOperation::MappingStandIn {
                                mapping_id: stand_in_id,
                            }),
                            second: Box::new(ValueOperation::from_second_layer_output(
                                primitive_output,
                                step,
                                map_id_supplier,
                                tables,
                            )),
                            operator: TwoValueOperator::Equal,
                        }
                    }
                };
                ValueOperation::TwoValueOp {
                    first: Box::new(ValueOperation::SetSize {
                        set: Box::new(SetOperation::MappedSet {
                            set: Box::new(SetOperation::from_second_layer(
                                set.as_ref(),
                                step,
                                map_id_supplier,
                                tables,
                            )),
                            stand_in_id,
                            mapping: Box::new(SetMapping::Value(mapping)),
                            filter: true,
                        }),
                    }),
                    second: Box::new(ValueOperation::FixedValue { value: 0 }),
                    operator: TwoValueOperator::Bigger,
                }
            }
            second_layer::steps::BooleanOutput::NumberComparison {
                first,
                second,
                operator,
            } => Self::TwoValueOp {
                first: Box::new(ValueOperation::from_second_layer_number_output(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(ValueOperation::from_second_layer_number_output(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    second_layer::steps::NumberComparor::Equal => TwoValueOperator::Equal,
                    second_layer::steps::NumberComparor::Unequal => TwoValueOperator::Unequal,
                    second_layer::steps::NumberComparor::Smaller => TwoValueOperator::Smaller,
                    second_layer::steps::NumberComparor::Bigger => TwoValueOperator::Bigger,
                    second_layer::steps::NumberComparor::SmallerEqual => {
                        TwoValueOperator::SmallerEqual
                    }
                    second_layer::steps::NumberComparor::BiggerEqual => {
                        TwoValueOperator::BiggerEqual
                    }
                },
            },
            second_layer::steps::BooleanOutput::EnumComparison {
                first,
                second,
                equal,
            } => Self::TwoValueOp {
                first: Box::new(ValueOperation::from_second_layer_enum_output(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(ValueOperation::from_second_layer_enum_output(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: if *equal {
                    TwoValueOperator::Equal
                } else {
                    TwoValueOperator::Unequal
                },
            },
            second_layer::steps::BooleanOutput::ObjectComparison {
                first,
                second,
                equal,
            } => {
                todo!()
            }
            second_layer::steps::BooleanOutput::ObjectFieldBoolean { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            second_layer::steps::BooleanOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: map_id_supplier.convert(*stand_in_id),
                }
            }
        }
    }
    fn from_second_layer_number_output(
        number_output: &second_layer::steps::NumberOutput,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match number_output {
            second_layer::steps::NumberOutput::Mapping => todo!(),
            second_layer::steps::NumberOutput::Number { value } => {
                ValueOperation::FixedValue { value: *value }
            }
            second_layer::steps::NumberOutput::MathOperation {
                first,
                second,
                operator,
            } => Self::TwoValueOp {
                first: Box::new(ValueOperation::from_second_layer_number_output(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(ValueOperation::from_second_layer_number_output(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    second_layer::steps::MathOperator::Add => TwoValueOperator::Add,
                    second_layer::steps::MathOperator::Subtr => TwoValueOperator::Subtr,
                    second_layer::steps::MathOperator::Mult => TwoValueOperator::Mult,
                    second_layer::steps::MathOperator::Div => TwoValueOperator::Div,
                    second_layer::steps::MathOperator::Mod => TwoValueOperator::Mod,
                    second_layer::steps::MathOperator::Rem => TwoValueOperator::Rem,
                    second_layer::steps::MathOperator::Expon => TwoValueOperator::Expon,
                },
            },
            second_layer::steps::NumberOutput::SetSize { set } => ValueOperation::SetSize {
                set: Box::new(SetOperation::from_second_layer(
                    set.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
            },
            second_layer::steps::NumberOutput::ObjectFieldNumber { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            second_layer::steps::NumberOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: map_id_supplier.convert(*stand_in_id),
                }
            }
        }
    }
    fn from_second_layer_enum_output(
        enum_output: &second_layer::steps::EnumOutput,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match enum_output {
            second_layer::steps::EnumOutput::Mapping => todo!(),
            second_layer::steps::EnumOutput::Enum { value } => {
                ValueOperation::FixedValue { value: todo!() }
            }
            second_layer::steps::EnumOutput::ObjectFieldEnum { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            second_layer::steps::EnumOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: map_id_supplier.convert(*stand_in_id),
                }
            }
        }
    }
    fn object_element_of_set(
        object: &second_layer::steps::ObjectOutput,
        set: &second_layer::steps::SetOutput,
        stand_in_id: usize,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match object {
            second_layer::steps::ObjectOutput::MappingStandIn {
                stand_in_id,
                partial,
                item_type,
            } => todo!(),
            second_layer::steps::ObjectOutput::StepObject { object_id, partial } => {
                if let Some(object) = step.get_step_objects().iter().find(|item| match item {
                    second_layer::steps::StepObject::BuildObject { id, fields: _ }
                    | second_layer::steps::StepObject::DeductionObject {
                        id,
                        table: _,
                        in_pool: _,
                        emmit_or_consum: _,
                    } => id == object_id,
                }) {
                    match object {
                        second_layer::steps::StepObject::DeductionObject {
                            id,
                            table: _,
                            in_pool: _,
                            emmit_or_consum: _,
                        } => todo!(),
                        second_layer::steps::StepObject::BuildObject { id, fields } => todo!(),
                    }
                } else {
                    panic!("No object of given id")
                }
            }
            second_layer::steps::ObjectOutput::FixedObject { fields } => fields.iter().fold(
                ValueOperation::FixedValue { value: 1 },
                |acc, (field_name, _id, field_value)| ValueOperation::TwoValueOp {
                    first: Box::new(acc),
                    second: Box::new(match field_value {
                        second_layer::steps::Output::Object(output) => {
                            todo!()
                        }
                        second_layer::steps::Output::Set(output) => {
                            if let (_, Some(second_value)) = ValueOperation::mapped_field_by_name(
                                set,
                                step,
                                field_name,
                                stand_in_id,
                                map_id_supplier,
                                tables,
                            ) {
                                ValueOperation::TwoSetOp {
                                    first: Box::new(SetOperation::from_second_layer(
                                        output.as_ref(),
                                        step,
                                        map_id_supplier,
                                        tables,
                                    )),
                                    second: Box::new(second_value),
                                    operator: ValueTwoSetOperator::Equal,
                                }
                            } else {
                                panic!()
                            }
                        }
                        second_layer::steps::Output::Primitive(primitive_output) => {
                            if let (Some(second_value), _) = ValueOperation::mapped_field_by_name(
                                set,
                                step,
                                field_name,
                                stand_in_id,
                                map_id_supplier,
                                tables,
                            ) {
                                ValueOperation::TwoValueOp {
                                    first: Box::new(ValueOperation::from_second_layer_output(
                                        primitive_output,
                                        step,
                                        map_id_supplier,
                                        tables,
                                    )),
                                    second: Box::new(second_value),
                                    operator: TwoValueOperator::Equal,
                                }
                            } else {
                                panic!("no primitive field in mapping")
                            }
                        }
                    }),
                    operator: TwoValueOperator::And,
                },
            ),
        }
    }
    fn field_value_from_second_layer(
        object: &Box<second_layer::steps::ObjectOutput>,
        field_id: &usize,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match object.as_ref() {
            second_layer::steps::ObjectOutput::MappingStandIn {
                stand_in_id,
                partial,
                item_type,
            } => {
                todo!()
            }
            second_layer::steps::ObjectOutput::StepObject {
                object_id,
                partial: _,
            } => ValueOperation::FieldValue {
                object_id: *object_id,
                field_id: *field_id,
            },
            second_layer::steps::ObjectOutput::FixedObject { fields } => {
                if let Some((_, _, output)) =
                    fields.iter().find(|(_, item_id, _)| item_id == field_id)
                {
                    match output {
                        second_layer::steps::Output::Primitive(primitive_output) => {
                            ValueOperation::from_second_layer_output(
                                primitive_output,
                                step,
                                map_id_supplier,
                                tables,
                            )
                        }
                        _ => panic!("inadequate field_type"),
                    }
                } else {
                    panic!("field missing in object")
                }
            }
        }
    }
    fn mapped_field_by_name(
        set: &second_layer::steps::SetOutput,
        step: &second_layer::steps::LogicStep,
        name: &PathString,
        mapping_id: usize,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> (Option<ValueOperation>, Option<SetOperation>) {
        match set {
            second_layer::steps::SetOutput::MappedSet {
                set: _,
                stand_in_id: _,
                mapping,
            } => match mapping {
                second_layer::steps::Output::Object(output) => Self::mapped_object_field_by_name(
                    output.as_ref(),
                    step,
                    name,
                    mapping_id,
                    map_id_supplier,
                    tables,
                ),
                _ => panic!("incompatible set output"),
            },
            second_layer::steps::SetOutput::FilteredSet {
                set,
                stand_in_id: _,
                filter: _,
            } => Self::mapped_field_by_name(set, step, name, mapping_id, map_id_supplier, tables),
            second_layer::steps::SetOutput::SetObject { set_object_id: _ } => {
                panic!("incompatible set output")
            }
            second_layer::steps::SetOutput::TwoSetOperation {
                first,
                second: _,
                operator: _,
            } => Self::mapped_field_by_name(first, step, name, mapping_id, map_id_supplier, tables),
            second_layer::steps::SetOutput::MultiSetOperation { set, operator: _ } => {
                Self::mapped_field_by_name(set, step, name, mapping_id, map_id_supplier, tables)
            }
            second_layer::steps::SetOutput::MappingStandIn {
                stand_in_id,
                item_type,
            } => todo!(),
        }
    }
    fn mapped_object_field_by_name(
        object: &second_layer::steps::ObjectOutput,
        step: &second_layer::steps::LogicStep,
        name: &PathString,
        mapping_id: usize,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> (Option<ValueOperation>, Option<SetOperation>) {
        match object {
            second_layer::steps::ObjectOutput::MappingStandIn {
                stand_in_id,
                partial,
                item_type,
            } => {
                if let Some((field_id, _)) = item_type
                    .object_fields(partial)
                    .iter()
                    .find(|(_, field_name)| field_name == name)
                {
                    match field_id {
                        FieldId::Primitive(field_id) => (
                            Some(ValueOperation::MappingStandInFieldValue {
                                mapping_id: map_id_supplier.convert(*stand_in_id),
                                field_id: *field_id,
                            }),
                            None,
                        ),
                        FieldId::Array(table_id) => (
                            None,
                            Some(SetOperation::MappingStandInFieldSet {
                                mapping_id: map_id_supplier.convert(*stand_in_id),
                                field_id: *table_id,
                            }),
                        ),
                    }
                } else {
                    panic!()
                }
            }
            second_layer::steps::ObjectOutput::StepObject { object_id, partial } => {
                if let Some(step_object) = step.get_step_objects().iter().find(|item| match item {
                    second_layer::steps::StepObject::DeductionObject {
                        id,
                        table: _,
                        in_pool: _,
                        emmit_or_consum: _,
                    }
                    | second_layer::steps::StepObject::BuildObject { id, fields: _ } => {
                        id == object_id
                    }
                }) {
                    if let Some((field_id, _)) = step_object
                        .get_object_fields(partial, tables)
                        .iter()
                        .find(|(_, field_name)| name == field_name)
                    {
                        match field_id {
                            FieldId::Primitive(field_id) => (
                                Some(ValueOperation::MappingStandInFieldValue {
                                    mapping_id,
                                    field_id: *field_id,
                                }),
                                None,
                            ),
                            FieldId::Array(table_id) => (
                                None,
                                Some(SetOperation::MappingStandInFieldSet {
                                    mapping_id,
                                    field_id: *table_id,
                                }),
                            ),
                        }
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            second_layer::steps::ObjectOutput::FixedObject { fields } => {
                if let Some(mapped_field) = fields.iter().find_map(|(field_name, id, output)| {
                    if name == field_name {
                        match output {
                            second_layer::steps::Output::Object(_output) => None,
                            second_layer::steps::Output::Set(output) => Some((
                                None,
                                Some(SetOperation::from_second_layer(
                                    output.as_ref(),
                                    step,
                                    map_id_supplier,
                                    tables,
                                )),
                            )),
                            second_layer::steps::Output::Primitive(primitive_output) => Some((
                                Some(ValueOperation::from_second_layer_output(
                                    primitive_output,
                                    step,
                                    map_id_supplier,
                                    tables,
                                )),
                                None,
                            )),
                        }
                    } else {
                        match output {
                            second_layer::steps::Output::Object(output) => {
                                let x = Self::mapped_object_field_by_name(
                                    output.as_ref(),
                                    step,
                                    name,
                                    mapping_id,
                                    map_id_supplier,
                                    tables,
                                );
                                if x.0.is_some() || x.1.is_some() {
                                    Some(x)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }
                }) {
                    mapped_field
                } else {
                    (None, None)
                }
            }
        }
    }
    fn from_second_layer_output(
        output: &second_layer::steps::PrimitiveOutput,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> ValueOperation {
        match output {
            second_layer::steps::PrimitiveOutput::Number(output) => {
                Self::from_second_layer_number_output(
                    output.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            second_layer::steps::PrimitiveOutput::Boolean(output) => {
                Self::from_second_layer_bool_output(output.as_ref(), step, map_id_supplier, tables)
            }

            second_layer::steps::PrimitiveOutput::Enum(output) => {
                Self::from_second_layer_enum_output(output.as_ref(), step, map_id_supplier, tables)
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum TwoValueOperator {
    And,
    Or,
    XOr,
    Equal,
    Unequal,
    Smaller,
    Bigger,
    SmallerEqual,
    BiggerEqual,
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Rem,
    Expon,
}
#[derive(Debug, Clone, Copy)]
enum ValueTwoSetOperator {
    Equal,
    SubsetOf,
    TrueSubsetOf,
    SupersetOf,
    TrueSupersetOf,
}

#[derive(Debug, Clone)]
enum SetOperation {
    MappedSet {
        set: Box<SetOperation>,
        stand_in_id: usize,
        mapping: Box<SetMapping>,
        filter: bool,
    },
    SetObject {
        set_object_id: usize,
    },
    FieldSet {
        object_id: usize,
        field_id: usize,
    },
    TwoSetOp {
        first: Box<SetOperation>,
        second: Box<SetOperation>,
        operator: SetTwoSetOperator,
    },
    MultiSetOp {
        set: Box<SetOperation>,
        operator: MultiSetOperator,
    },
    MappingStandIn {
        mapping_id: usize,
    },
    MappingStandInFieldSet {
        mapping_id: usize,
        field_id: TableId,
    },
}
impl SetOperation {
    fn from_second_layer(
        set_output: &second_layer::steps::SetOutput,
        step: &second_layer::steps::LogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &(
            Vec<&second_layer::tables::DeductionTable>,
            Vec<&second_layer::tables::ArrayTable>,
        ),
    ) -> SetOperation {
        match set_output {
            second_layer::steps::SetOutput::MappedSet {
                set,
                stand_in_id,
                mapping,
            } => Self::MappedSet {
                set: Box::new(Self::from_second_layer(
                    set.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                stand_in_id: map_id_supplier.convert(*stand_in_id),
                mapping: {
                    match mapping {
                        second_layer::steps::Output::Object(output) => {
                            todo!()
                        }
                        second_layer::steps::Output::Set(output) => Box::new(SetMapping::Set(
                            Self::from_second_layer(output.as_ref(), step, map_id_supplier, tables),
                        )),
                        second_layer::steps::Output::Primitive(output) => {
                            Box::new(SetMapping::Value(ValueOperation::from_second_layer_output(
                                output,
                                step,
                                map_id_supplier,
                                tables,
                            )))
                        }
                    }
                },
                filter: false,
            },
            second_layer::steps::SetOutput::FilteredSet {
                set,
                stand_in_id,
                filter,
            } => Self::MappedSet {
                set: Box::new(Self::from_second_layer(
                    set.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                stand_in_id: map_id_supplier.convert(*stand_in_id),
                mapping: {
                    Box::new(SetMapping::Value(
                        ValueOperation::from_second_layer_bool_output(
                            filter,
                            step,
                            map_id_supplier,
                            tables,
                        ),
                    ))
                },
                filter: false,
            },
            second_layer::steps::SetOutput::SetObject { set_object_id } => Self::SetObject {
                set_object_id: *set_object_id,
            },
            second_layer::steps::SetOutput::TwoSetOperation {
                first,
                second,
                operator,
            } => Self::TwoSetOp {
                first: Box::new(Self::from_second_layer(
                    first.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                second: Box::new(Self::from_second_layer(
                    second.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    second_layer::steps::TwoSetOperator::Union => SetTwoSetOperator::Union,
                    second_layer::steps::TwoSetOperator::Intersect => SetTwoSetOperator::Intersect,
                    second_layer::steps::TwoSetOperator::Without => SetTwoSetOperator::Without,
                    second_layer::steps::TwoSetOperator::SubtractedFrom => {
                        SetTwoSetOperator::SubtractedFrom
                    }
                    second_layer::steps::TwoSetOperator::DisjointWith => {
                        SetTwoSetOperator::DisjointWith
                    }
                    second_layer::steps::TwoSetOperator::DisjunctiveUnion => {
                        SetTwoSetOperator::DisjunctiveUnion
                    }
                },
            },
            second_layer::steps::SetOutput::MultiSetOperation { set, operator } => {
                Self::MultiSetOp {
                    set: Box::new(Self::from_second_layer(
                        set.as_ref(),
                        step,
                        map_id_supplier,
                        tables,
                    )),
                    operator: match operator {
                        second_layer::steps::MultiSetOperator::Union => MultiSetOperator::Union,
                        second_layer::steps::MultiSetOperator::Intersect => {
                            MultiSetOperator::Intersect
                        }
                    },
                }
            }
            second_layer::steps::SetOutput::MappingStandIn {
                stand_in_id,
                item_type: _,
            } => Self::MappingStandIn {
                mapping_id: map_id_supplier.convert(*stand_in_id),
            },
        }
    }
}
#[derive(Debug, Clone)]
enum SetMapping {
    Value(ValueOperation),
    FixedObject(BuildObjectFields, ValueOperation),
    Set(SetOperation),
}
#[derive(Debug, Clone, Copy)]
enum SetTwoSetOperator {
    Union,
    Intersect,
    Without,
    SubtractedFrom,
    DisjointWith,
    DisjunctiveUnion,
}
#[derive(Debug, Clone, Copy)]
enum MultiSetOperator {
    Union,
    Intersect,
}
