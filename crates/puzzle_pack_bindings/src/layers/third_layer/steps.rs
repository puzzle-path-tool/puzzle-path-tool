use itertools::Itertools;

use crate::layers::{
    id_helpers::{EnumTableId as LookUpTableId, FieldId, IdSupplier, PathString, StepId, TableId},
    second_layer::{
        steps::{
            BoolCombinator as SecondLayerBoolComb, BooleanOutput as SecondLayerBoolean,
            BuildObjectFields as SecondLayerObjectFields, EnumOutput as SecondLayerEnum,
            LogicStep as SecondLayerLogicStep, MathOperator as SecondLayerMathOp,
            MultiSetOperator as SecondLayerMultiSetOp, NumberComparor as SecondLayerNumberComp,
            NumberOutput as SecondLayerNumber, ObjectOutput as SecondLayerObject,
            Output as SecondLayerOutput, PrimitiveOutput as SecondLayerPrimitive,
            SetComparor as SecondLayerSetComp, SetObject as SecondLayerSetObject,
            SetOutput as SecondLayerSet, StepObject as SecondLayerStepObject,
            TwoSetOperator as SecondLayerTwoSetOp,
        },
        tables::TableBundle as SecondLayerTables,
    },
};

#[derive(Debug, Clone)]
pub struct LogicStep {
    id: StepId,
    match_statement: ValueOperation,
    step_objects: Vec<StepObject>,
    step_sets: Vec<SetObject>,
}
impl LogicStep {
    pub(crate) fn from_second_layer(
        logic_step: &SecondLayerLogicStep,
        tables: &SecondLayerTables,
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
    pub fn get_id(&self) -> StepId {
        self.id
    }
    pub fn get_match_statement(&self) -> &ValueOperation {
        &self.match_statement
    }
    pub fn get_step_objects(&self) -> &Vec<StepObject> {
        &self.step_objects
    }
    pub fn get_step_sets(&self) -> &Vec<SetObject> {
        &self.step_sets
    }
}

#[derive(Debug, Clone)]
pub enum StepObject {
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
    fn from_second_layer(object: &SecondLayerStepObject) -> StepObject {
        match object {
            SecondLayerStepObject::DeductionObject {
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
            SecondLayerStepObject::BuildObject { id, fields } => StepObject::BuildObject {
                id: *id,
                fields: BuildObjectFields::from_second_layer(fields),
            },
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildObjectFields {
    field_size: usize,                        //Vec<(FieldId, PathString)>,
    array_fields: Vec<BuildObjectArrayField>, //Vec<(FieldId, PathString)>)>,
}
impl BuildObjectFields {
    fn from_second_layer(fields: &SecondLayerObjectFields) -> BuildObjectFields {
        let (value_fields, array_fields) = fields.flat_type();
        let value_fields = Self::get_highest_id(&value_fields);
        let array_fields = array_fields
            .iter()
            .map(|(id, ref_ids, fields)| BuildObjectArrayField {
                id: *id,
                field_size: Self::get_highest_id(fields),
                ref_ids: ref_ids.clone(),
            })
            .collect();
        BuildObjectFields {
            field_size: value_fields,
            array_fields,
        }
    }
    fn get_highest_id(fields: &Vec<(FieldId, PathString)>) -> usize {
        fields.iter().fold(0, |acc, (field_id, _)| match field_id {
            FieldId::Primitive(id) => std::cmp::max(*id, acc),
            FieldId::Array(_) => acc,
        })
    }
    pub fn get_field_size(&self) -> usize {
        self.field_size
    }
    pub fn get_array_fields(&self) -> &Vec<BuildObjectArrayField> {
        &self.array_fields
    }
}
#[derive(Debug, Clone)]
pub struct BuildObjectArrayField {
    id: TableId,
    field_size: usize,
    ref_ids: Vec<TableId>,
}
impl BuildObjectArrayField {
    pub fn get_id(&self) -> TableId {self.id}
    pub fn get_field_size(&self) -> usize {
        self.field_size
    }
    pub fn get_ref_ids(&self) -> &Vec<TableId> {
        &self.ref_ids
    }
    pub fn get_array_depth(&self) -> usize {
        self.ref_ids.len()
    }
}
#[derive(Debug, Clone)]
enum SetObject {
    SetOfObjects(StepObject),
    SetOfSets(Box<SetObject>),
}
impl SetObject {
    fn from_second_layer(set: &SecondLayerSetObject) -> SetObject {
        match set {
            SecondLayerSetObject::SetOfObjects(step_object) => {
                Self::SetOfObjects(StepObject::from_second_layer(step_object))
            }
            SecondLayerSetObject::SetOfSets(set_object) => {
                Self::SetOfSets(Box::new(SetObject::from_second_layer(set_object.as_ref())))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueOperation {
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
    MappedValue {
        mapping_id: usize,
    },
    MappedObjectFieldValue {
        mapping_id: usize,
        field_id: usize,
    },
    ConvertedValue {
        converted_value: Box<ValueOperation>,
        look_up_table: LookUpTableId,
    },
}
impl ValueOperation {
    fn from_second_layer_bool_output(
        bool_output: &SecondLayerBoolean,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match bool_output {
            SecondLayerBoolean::Boolean { value } => ValueOperation::FixedValue {
                value: i32::from(*value),
            },
            SecondLayerBoolean::BoolCombination {
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
                    SecondLayerBoolComb::And => TwoValueOperator::Add,
                    SecondLayerBoolComb::Or => TwoValueOperator::Or,
                    SecondLayerBoolComb::XOr => TwoValueOperator::XOr,
                },
            },
            SecondLayerBoolean::SetComparison {
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
                    SecondLayerSetComp::Equal => ValueTwoSetOperator::Equal,
                    SecondLayerSetComp::SubsetOf => ValueTwoSetOperator::SubsetOf,
                    SecondLayerSetComp::TrueSubsetOf => ValueTwoSetOperator::TrueSubsetOf,
                    SecondLayerSetComp::SupersetOf => ValueTwoSetOperator::SupersetOf,
                    SecondLayerSetComp::TrueSupersetOf => ValueTwoSetOperator::TrueSupersetOf,
                },
            },
            SecondLayerBoolean::ElementOfSet { element, set } => {
                let stand_in_id = map_id_supplier.next();
                let mapping = match element.as_ref() {
                    SecondLayerOutput::Object(output) => ValueOperation::object_element_of_set(
                        output,
                        set,
                        stand_in_id,
                        step,
                        map_id_supplier,
                        tables,
                    ),
                    SecondLayerOutput::Set(output) => ValueOperation::TwoSetOp {
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
                    SecondLayerOutput::Primitive(primitive_output) => ValueOperation::TwoValueOp {
                        first: Box::new(ValueOperation::MappedValue {
                            mapping_id: stand_in_id,
                        }),
                        second: Box::new(ValueOperation::from_second_layer_output(
                            primitive_output,
                            step,
                            map_id_supplier,
                            tables,
                        )),
                        operator: TwoValueOperator::Equal,
                    },
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
                            mapping_id: stand_in_id,
                            mapping: Box::new(SetMapping::SingleValue(SetMappingValue::Value(
                                mapping,
                            ))),
                            filter: true,
                        }),
                    }),
                    second: Box::new(ValueOperation::FixedValue { value: 0 }),
                    operator: TwoValueOperator::Bigger,
                }
            }
            SecondLayerBoolean::NumberComparison {
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
                    SecondLayerNumberComp::Equal => TwoValueOperator::Equal,
                    SecondLayerNumberComp::Unequal => TwoValueOperator::Unequal,
                    SecondLayerNumberComp::Smaller => TwoValueOperator::Smaller,
                    SecondLayerNumberComp::Bigger => TwoValueOperator::Bigger,
                    SecondLayerNumberComp::SmallerEqual => TwoValueOperator::SmallerEqual,
                    SecondLayerNumberComp::BiggerEqual => TwoValueOperator::BiggerEqual,
                },
            },
            SecondLayerBoolean::EnumComparison {
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
            SecondLayerBoolean::ObjectComparison {
                first,
                second,
                equal,
            } => ValueOperation::compare_objects(
                first.as_ref(),
                second.as_ref(),
                *equal,
                step,
                map_id_supplier,
                tables,
            ),
            SecondLayerBoolean::ObjectFieldBoolean { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            SecondLayerBoolean::MappingStandIn { stand_in_id } => ValueOperation::MappedValue {
                mapping_id: map_id_supplier.convert(*stand_in_id),
            },
        }
    }
    fn from_second_layer_number_output(
        number_output: &SecondLayerNumber,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match number_output {
            SecondLayerNumber::Mapping {
                mapped_enum,
                mapping_table,
            } => ValueOperation::ConvertedValue {
                converted_value: Box::new(ValueOperation::from_second_layer_enum_output(
                    mapped_enum.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                look_up_table: *mapping_table,
            },
            SecondLayerNumber::Number { value } => ValueOperation::FixedValue { value: *value },
            SecondLayerNumber::MathOperation {
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
                    SecondLayerMathOp::Add => TwoValueOperator::Add,
                    SecondLayerMathOp::Subtr => TwoValueOperator::Subtr,
                    SecondLayerMathOp::Mult => TwoValueOperator::Mult,
                    SecondLayerMathOp::Div => TwoValueOperator::Div,
                    SecondLayerMathOp::Mod => TwoValueOperator::Mod,
                    SecondLayerMathOp::Rem => TwoValueOperator::Rem,
                    SecondLayerMathOp::Expon => TwoValueOperator::Expon,
                },
            },
            SecondLayerNumber::SetSize { set } => ValueOperation::SetSize {
                set: Box::new(SetOperation::from_second_layer(
                    set.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
            },
            SecondLayerNumber::ObjectFieldNumber { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            SecondLayerNumber::MappingStandIn { stand_in_id } => ValueOperation::MappedValue {
                mapping_id: map_id_supplier.convert(*stand_in_id),
            },
        }
    }
    fn from_second_layer_enum_output(
        enum_output: &SecondLayerEnum,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match enum_output {
            SecondLayerEnum::Mapping {
                mapped_number,
                mapping_table,
            } => ValueOperation::ConvertedValue {
                converted_value: Box::new(ValueOperation::from_second_layer_number_output(
                    mapped_number.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                look_up_table: *mapping_table,
            },
            SecondLayerEnum::Enum { value, enum_id } => {
                if let Some(value) = tables
                    .get_enum_by_id(*enum_id)
                    .and_then(|enum_table| enum_table.convert(value))
                {
                    ValueOperation::FixedValue {
                        value: i32::from(value),
                    }
                } else {
                    panic!("Enumtable or conversion not found")
                }
            }
            SecondLayerEnum::ObjectFieldEnum { object, field_id } => {
                ValueOperation::field_value_from_second_layer(
                    object,
                    field_id,
                    step,
                    map_id_supplier,
                    tables,
                )
            }
            SecondLayerEnum::MappingStandIn { stand_in_id } => ValueOperation::MappedValue {
                mapping_id: map_id_supplier.convert(*stand_in_id),
            },
        }
    }
    fn object_element_of_set(
        object: &SecondLayerObject,
        set: &SecondLayerSet,
        mapping_id: usize,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match object {
            SecondLayerObject::MappingStandIn {
                stand_in_id,
                partial,
                item_type,
            } => {
                let fields = item_type.object_fields(partial);
                let matches = fields.iter().map(|(field_id, field_name)| {
                    let mapped_field = ValueOperation::mapped_field_by_name(
                        set,
                        step,
                        field_name,
                        mapping_id,
                        map_id_supplier,
                        tables,
                    );
                    match field_id {
                        FieldId::Primitive(field_id) => {
                            if let (Some(second_value), _) = mapped_field {
                                ValueOperation::TwoValueOp {
                                    first: Box::new(ValueOperation::MappedObjectFieldValue {
                                        mapping_id: map_id_supplier.convert(*stand_in_id),
                                        field_id: *field_id,
                                    }),
                                    second: Box::new(second_value),
                                    operator: TwoValueOperator::Equal,
                                }
                            } else {
                                panic!("no primitive field in mapping")
                            }
                        }
                        FieldId::Array(table_id) => {
                            if let (_, Some(second_set)) = mapped_field {
                                ValueOperation::TwoSetOp {
                                    first: Box::new(SetOperation::MappingStandInFieldSet {
                                        mapping_id: map_id_supplier.convert(*stand_in_id),
                                        field_id: *table_id,
                                    }),
                                    second: Box::new(second_set),
                                    operator: ValueTwoSetOperator::Equal,
                                }
                            } else {
                                panic!("no set field in mapping")
                            }
                        }
                    }
                });
                ValueOperation::all_true(matches.collect())
            }
            SecondLayerObject::StepObject { object_id, partial } => {
                if let Some(object) = step.get_setp_object_by_id(*object_id) {
                    let fields = object.get_object_fields(partial, tables);
                    let matches = fields.iter().map(|(field_id, field_name)| {
                        let mapped_field = ValueOperation::mapped_field_by_name(
                            set,
                            step,
                            field_name,
                            mapping_id,
                            map_id_supplier,
                            tables,
                        );
                        match field_id {
                            FieldId::Primitive(field_id) => {
                                if let (Some(second_value), _) = mapped_field {
                                    ValueOperation::TwoValueOp {
                                        first: Box::new(ValueOperation::FieldValue {
                                            object_id: *object_id,
                                            field_id: *field_id,
                                        }),
                                        second: Box::new(second_value),
                                        operator: TwoValueOperator::Equal,
                                    }
                                } else {
                                    panic!("no primitive field in mapping")
                                }
                            }
                            FieldId::Array(table_id) => {
                                if let (_, Some(second_set)) = mapped_field {
                                    ValueOperation::TwoSetOp {
                                        first: Box::new(SetOperation::FieldSet {
                                            object_id: *object_id,
                                            field_id: *table_id,
                                        }),
                                        second: Box::new(second_set),
                                        operator: ValueTwoSetOperator::Equal,
                                    }
                                } else {
                                    panic!("no set field in mapping")
                                }
                            }
                        }
                    });
                    ValueOperation::all_true(matches.collect())
                } else {
                    panic!("No object of given id")
                }
            }
            SecondLayerObject::FixedObject { fields } => {
                let matches =
                    fields
                        .iter()
                        .map(|(field_name, _id, field_value)| match field_value {
                            SecondLayerOutput::Object(output) => {
                                ValueOperation::object_element_of_set(
                                    output.as_ref(),
                                    set,
                                    mapping_id,
                                    step,
                                    map_id_supplier,
                                    tables,
                                )
                            }
                            SecondLayerOutput::Set(output) => {
                                if let (_, Some(second_value)) =
                                    ValueOperation::mapped_field_by_name(
                                        set,
                                        step,
                                        field_name,
                                        mapping_id,
                                        map_id_supplier,
                                        tables,
                                    )
                                {
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
                            SecondLayerOutput::Primitive(primitive_output) => {
                                if let (Some(second_value), _) =
                                    ValueOperation::mapped_field_by_name(
                                        set,
                                        step,
                                        field_name,
                                        mapping_id,
                                        map_id_supplier,
                                        tables,
                                    )
                                {
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
                        });
                ValueOperation::all_true(matches.collect())
            }
        }
    }
    fn compare_objects(
        first: &SecondLayerObject,
        second: &SecondLayerObject,
        equal: bool,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        let matches: Vec<ValueOperation> = match first {
            SecondLayerObject::MappingStandIn {
                stand_in_id,
                partial,
                item_type,
            } => {
                let fields = item_type.object_fields(partial);
                fields
                    .iter()
                    .map(|(field_id, field_name)| {
                        let field = ValueOperation::object_field_by_name(
                            second,
                            step,
                            field_name,
                            map_id_supplier,
                            tables,
                        );
                        match field_id {
                            FieldId::Primitive(field_id) => {
                                if let (Some(second_value), _) = field {
                                    ValueOperation::TwoValueOp {
                                        first: Box::new(ValueOperation::MappedObjectFieldValue {
                                            mapping_id: map_id_supplier.convert(*stand_in_id),
                                            field_id: *field_id,
                                        }),
                                        second: Box::new(second_value),
                                        operator: TwoValueOperator::Equal,
                                    }
                                } else {
                                    panic!("no primitive field in Object")
                                }
                            }
                            FieldId::Array(table_id) => {
                                if let (_, Some(second_set)) = field {
                                    ValueOperation::TwoSetOp {
                                        first: Box::new(SetOperation::MappingStandInFieldSet {
                                            mapping_id: map_id_supplier.convert(*stand_in_id),
                                            field_id: *table_id,
                                        }),
                                        second: Box::new(second_set),
                                        operator: ValueTwoSetOperator::Equal,
                                    }
                                } else {
                                    panic!("no set field in mapping")
                                }
                            }
                        }
                    })
                    .collect()
            }
            SecondLayerObject::StepObject { object_id, partial } => {
                if let Some(object) = step
                    .get_step_objects()
                    .iter()
                    .find(|item| item.get_id() == *object_id)
                {
                    let fields = object.get_object_fields(partial, tables);
                    fields
                        .iter()
                        .map(|(field_id, field_name)| {
                            let field = ValueOperation::object_field_by_name(
                                second,
                                step,
                                field_name,
                                map_id_supplier,
                                tables,
                            );
                            match field_id {
                                FieldId::Primitive(field_id) => {
                                    if let (Some(second_value), _) = field {
                                        ValueOperation::TwoValueOp {
                                            first: Box::new(ValueOperation::FieldValue {
                                                object_id: *object_id,
                                                field_id: *field_id,
                                            }),
                                            second: Box::new(second_value),
                                            operator: TwoValueOperator::Equal,
                                        }
                                    } else {
                                        panic!("no primitive field in mapping")
                                    }
                                }
                                FieldId::Array(table_id) => {
                                    if let (_, Some(second_set)) = field {
                                        ValueOperation::TwoSetOp {
                                            first: Box::new(SetOperation::FieldSet {
                                                object_id: *object_id,
                                                field_id: *table_id,
                                            }),
                                            second: Box::new(second_set),
                                            operator: ValueTwoSetOperator::Equal,
                                        }
                                    } else {
                                        panic!("no set field in mapping")
                                    }
                                }
                            }
                        })
                        .collect()
                } else {
                    panic!("No object of given id")
                }
            }
            SecondLayerObject::FixedObject { fields } => fields
                .iter()
                .map(|(field_name, _id, field_value)| match field_value {
                    SecondLayerOutput::Object(output) => ValueOperation::compare_objects(
                        output.as_ref(),
                        second,
                        true,
                        step,
                        map_id_supplier,
                        tables,
                    ),
                    SecondLayerOutput::Set(output) => {
                        if let (_, Some(second_value)) = ValueOperation::object_field_by_name(
                            second,
                            step,
                            field_name,
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
                    SecondLayerOutput::Primitive(primitive_output) => {
                        if let (Some(second_value), _) = ValueOperation::object_field_by_name(
                            second,
                            step,
                            field_name,
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
                })
                .collect(),
        };
        let matches = ValueOperation::all_true(matches);
        if equal {
            matches
        } else {
            ValueOperation::TwoValueOp {
                first: Box::new(matches),
                second: Box::new(ValueOperation::FixedValue { value: 0 }),
                operator: TwoValueOperator::Equal,
            }
        }
    }
    fn field_value_from_second_layer(
        object: &Box<SecondLayerObject>,
        field_id: &usize,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match object.as_ref() {
            SecondLayerObject::MappingStandIn {
                stand_in_id,
                partial: _,
                item_type: _,
            } => ValueOperation::MappedObjectFieldValue {
                mapping_id: map_id_supplier.convert(*stand_in_id),
                field_id: *field_id,
            },
            SecondLayerObject::StepObject {
                object_id,
                partial: _,
            } => ValueOperation::FieldValue {
                object_id: *object_id,
                field_id: *field_id,
            },
            SecondLayerObject::FixedObject { fields } => {
                if let Some((_, _, output)) = fields
                    .iter()
                    .find(|(_, item_id, _)| item_id == &Some(FieldId::Primitive(*field_id)))
                {
                    match output {
                        SecondLayerOutput::Primitive(primitive_output) => {
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
        set: &SecondLayerSet,
        step: &SecondLayerLogicStep,
        name: &PathString,
        mapping_id: usize,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> (Option<ValueOperation>, Option<SetOperation>) {
        match set {
            SecondLayerSet::MappedSet {
                set: _,
                stand_in_id: _,
                mapping,
            } => match mapping {
                SecondLayerOutput::Object(output) => Self::mapped_object_field_by_name(
                    output.as_ref(),
                    step,
                    name,
                    mapping_id,
                    map_id_supplier,
                    tables,
                ),
                _ => panic!("incompatible set output"),
            },
            SecondLayerSet::FilteredSet {
                set,
                stand_in_id: _,
                filter: _,
            } => Self::mapped_field_by_name(set, step, name, mapping_id, map_id_supplier, tables),
            SecondLayerSet::SetObject { set_object_id } => {
                if let Some(set_object) = step
                    .get_step_sets()
                    .iter()
                    .find(|set_object| set_object.get_id() == *set_object_id)
                {
                    match set_object {
                        SecondLayerSetObject::SetOfObjects(step_object) => {
                            if let Some((field_id, _)) = step_object
                                .get_object_fields(&None, tables)
                                .iter()
                                .find(|(_, field_name)| field_name == name)
                            {
                                match field_id {
                                    FieldId::Primitive(primitive_id) => (
                                        Some(ValueOperation::MappedObjectFieldValue {
                                            mapping_id,
                                            field_id: *primitive_id,
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
                                (None, None)
                            }
                        }
                        SecondLayerSetObject::SetOfSets(_set_object) => {
                            panic!("incompatible set output")
                        }
                    }
                } else {
                    (None, None)
                }
            }
            SecondLayerSet::TwoSetOperation {
                first,
                second: _,
                operator: _,
            } => Self::mapped_field_by_name(first, step, name, mapping_id, map_id_supplier, tables),
            SecondLayerSet::MultiSetOperation { set, operator: _ } => {
                Self::mapped_field_by_name(set, step, name, mapping_id, map_id_supplier, tables)
            }
            SecondLayerSet::MappingStandIn {
                stand_in_id: _,
                item_type,
                set_in_set_depth,
            } => {
                if set_in_set_depth > &0 {
                    panic!("incompatible set output")
                } else if let Some((field_id, _)) = item_type
                    .object_fields(&None)
                    .iter()
                    .find(|(_, field_name)| field_name == name)
                {
                    match field_id {
                        FieldId::Primitive(primitive_id) => (
                            Some(ValueOperation::MappedObjectFieldValue {
                                mapping_id,
                                field_id: *primitive_id,
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
                    (None, None)
                }
            }
        }
    }
    fn mapped_object_field_by_name(
        object: &SecondLayerObject,
        step: &SecondLayerLogicStep,
        name: &PathString,
        mapping_id: usize,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> (Option<ValueOperation>, Option<SetOperation>) {
        match object {
            SecondLayerObject::MappingStandIn {
                stand_in_id: _,
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
                            Some(ValueOperation::MappedObjectFieldValue {
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
            }
            SecondLayerObject::StepObject { object_id, partial } => {
                if let Some(step_object) = step
                    .get_step_objects()
                    .iter()
                    .find(|item| item.get_id() == *object_id)
                {
                    if let Some((field_id, _)) = step_object
                        .get_object_fields(partial, tables)
                        .iter()
                        .find(|(_, field_name)| name == field_name)
                    {
                        match field_id {
                            FieldId::Primitive(field_id) => (
                                Some(ValueOperation::MappedObjectFieldValue {
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
            SecondLayerObject::FixedObject { fields } => {
                if let Some(mapped_field) = fields.iter().find_map(|(field_name, _id, output)| {
                    if name == field_name {
                        match output {
                            SecondLayerOutput::Object(_output) => None,
                            SecondLayerOutput::Set(output) => Some((
                                None,
                                Some(SetOperation::from_second_layer(
                                    output.as_ref(),
                                    step,
                                    map_id_supplier,
                                    tables,
                                )),
                            )),
                            SecondLayerOutput::Primitive(primitive_output) => Some((
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
                            SecondLayerOutput::Object(output) => {
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
    fn object_field_by_name(
        object: &SecondLayerObject,
        step: &SecondLayerLogicStep,
        name: &PathString,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> (Option<ValueOperation>, Option<SetOperation>) {
        match object {
            SecondLayerObject::MappingStandIn {
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
                            Some(ValueOperation::MappedObjectFieldValue {
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
            SecondLayerObject::StepObject { object_id, partial } => {
                if let Some(step_object) = step
                    .get_step_objects()
                    .iter()
                    .find(|item| item.get_id() == *object_id)
                {
                    if let Some((field_id, _)) = step_object
                        .get_object_fields(partial, tables)
                        .iter()
                        .find(|(_, field_name)| name == field_name)
                    {
                        match field_id {
                            FieldId::Primitive(field_id) => (
                                Some(ValueOperation::FieldValue {
                                    object_id: *object_id,
                                    field_id: *field_id,
                                }),
                                None,
                            ),
                            FieldId::Array(table_id) => (
                                None,
                                Some(SetOperation::FieldSet {
                                    object_id: *object_id,
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
            SecondLayerObject::FixedObject { fields } => {
                if let Some(field) = fields.iter().find_map(|(field_name, _id, output)| {
                    if name == field_name {
                        match output {
                            SecondLayerOutput::Object(_output) => None,
                            SecondLayerOutput::Set(output) => Some((
                                None,
                                Some(SetOperation::from_second_layer(
                                    output.as_ref(),
                                    step,
                                    map_id_supplier,
                                    tables,
                                )),
                            )),
                            SecondLayerOutput::Primitive(primitive_output) => Some((
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
                            SecondLayerOutput::Object(output) => {
                                let x = Self::object_field_by_name(
                                    output.as_ref(),
                                    step,
                                    name,
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
                    field
                } else {
                    (None, None)
                }
            }
        }
    }
    fn from_second_layer_output(
        output: &SecondLayerPrimitive,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> ValueOperation {
        match output {
            SecondLayerPrimitive::Number(output) => Self::from_second_layer_number_output(
                output.as_ref(),
                step,
                map_id_supplier,
                tables,
            ),
            SecondLayerPrimitive::Boolean(output) => {
                Self::from_second_layer_bool_output(output.as_ref(), step, map_id_supplier, tables)
            }

            SecondLayerPrimitive::Enum(output) => {
                Self::from_second_layer_enum_output(output.as_ref(), step, map_id_supplier, tables)
            }
        }
    }
    fn all_true(list: Vec<Self>) -> Self {
        list.iter()
            .fold(ValueOperation::FixedValue { value: 1 }, |acc, item| {
                ValueOperation::TwoValueOp {
                    first: Box::new(acc),
                    second: Box::new(item.to_owned()),
                    operator: TwoValueOperator::And,
                }
            })
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TwoValueOperator {
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
pub enum ValueTwoSetOperator {
    Equal,
    SubsetOf,
    TrueSubsetOf,
    SupersetOf,
    TrueSupersetOf,
}

#[derive(Debug, Clone)]
pub enum SetOperation {
    MappedSet {
        set: Box<SetOperation>,
        mapping_id: usize,
        mapping: Box<SetMapping>,
        filter: bool,
    },
    SetObject {
        set_object_id: usize,
    },
    FieldSet {
        object_id: usize,
        field_id: TableId,
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
        set_output: &SecondLayerSet,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> SetOperation {
        match set_output {
            SecondLayerSet::MappedSet {
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
                mapping_id: map_id_supplier.convert(*stand_in_id),
                mapping: Box::new(SetMapping::from_second_layer_output(
                    mapping,
                    step,
                    map_id_supplier,
                    tables,
                )),
                filter: false,
            },
            SecondLayerSet::FilteredSet {
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
                mapping_id: map_id_supplier.convert(*stand_in_id),
                mapping: {
                    Box::new(SetMapping::SingleValue(SetMappingValue::Value(
                        ValueOperation::from_second_layer_bool_output(
                            filter,
                            step,
                            map_id_supplier,
                            tables,
                        ),
                    )))
                },
                filter: false,
            },
            SecondLayerSet::SetObject { set_object_id } => Self::SetObject {
                set_object_id: *set_object_id,
            },
            SecondLayerSet::TwoSetOperation {
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
                    SecondLayerTwoSetOp::Union => SetTwoSetOperator::Union,
                    SecondLayerTwoSetOp::Intersect => SetTwoSetOperator::Intersect,
                    SecondLayerTwoSetOp::Without => SetTwoSetOperator::Without,
                    SecondLayerTwoSetOp::SubtractedFrom => SetTwoSetOperator::SubtractedFrom,
                    SecondLayerTwoSetOp::DisjointWith => SetTwoSetOperator::DisjointWith,
                    SecondLayerTwoSetOp::DisjunctiveUnion => SetTwoSetOperator::DisjunctiveUnion,
                },
            },
            SecondLayerSet::MultiSetOperation { set, operator } => Self::MultiSetOp {
                set: Box::new(Self::from_second_layer(
                    set.as_ref(),
                    step,
                    map_id_supplier,
                    tables,
                )),
                operator: match operator {
                    SecondLayerMultiSetOp::Union => MultiSetOperator::Union,
                    SecondLayerMultiSetOp::Intersect => MultiSetOperator::Intersect,
                },
            },
            SecondLayerSet::MappingStandIn {
                stand_in_id,
                item_type: _,
                set_in_set_depth: _,
            } => Self::MappingStandIn {
                mapping_id: map_id_supplier.convert(*stand_in_id),
            },
        }
    }
}
#[derive(Debug, Clone)]
enum SetMapping {
    SingleValue(SetMappingValue),
    FixedObject(Vec<(FieldId, SetMappingValue)>),
}
#[derive(Debug, Clone)]
enum SetMappingValue {
    Value(ValueOperation),
    Set(SetOperation),
}
impl SetMapping {
    fn from_second_layer_output(
        mapping: &SecondLayerOutput,
        step: &SecondLayerLogicStep,
        map_id_supplier: &mut IdSupplier,
        tables: &SecondLayerTables,
    ) -> SetMapping {
        match mapping {
            SecondLayerOutput::Object(output) => SetMapping::FixedObject(match output.as_ref() {
                SecondLayerObject::MappingStandIn {
                    stand_in_id,
                    partial,
                    item_type,
                } => item_type
                    .object_fields(partial)
                    .iter()
                    .map(|(field_id, _)| match field_id {
                        FieldId::Primitive(primitive_id) => (
                            *field_id,
                            SetMappingValue::Value(ValueOperation::MappedObjectFieldValue {
                                mapping_id: map_id_supplier.convert(*stand_in_id),
                                field_id: *primitive_id,
                            }),
                        ),
                        FieldId::Array(table_id) => (
                            *field_id,
                            SetMappingValue::Set(SetOperation::MappingStandInFieldSet {
                                mapping_id: map_id_supplier.convert(*stand_in_id),
                                field_id: *table_id,
                            }),
                        ),
                    })
                    .collect(),
                SecondLayerObject::StepObject { object_id, partial } => {
                    if let Some(step_object) = step.get_setp_object_by_id(*object_id) {
                        step_object
                            .get_object_fields(partial, tables)
                            .iter()
                            .map(|(field_id, _)| match field_id {
                                FieldId::Primitive(primitive_id) => (
                                    *field_id,
                                    SetMappingValue::Value(ValueOperation::FieldValue {
                                        object_id: *object_id,
                                        field_id: *primitive_id,
                                    }),
                                ),
                                FieldId::Array(table_id) => (
                                    *field_id,
                                    SetMappingValue::Set(SetOperation::FieldSet {
                                        object_id: *object_id,
                                        field_id: *table_id,
                                    }),
                                ),
                            })
                            .collect()
                    } else {
                        panic!()
                    }
                }
                SecondLayerObject::FixedObject { fields } => fields
                    .iter()
                    .map(|(_, field_id, output)| {
                        match SetMapping::from_second_layer_output(
                            output,
                            step,
                            map_id_supplier,
                            tables,
                        ) {
                            SetMapping::SingleValue(set_mapping_value) => {
                                if let Some(field_id) = field_id {
                                    vec![(*field_id, set_mapping_value)]
                                } else {
                                    panic!()
                                }
                            }
                            SetMapping::FixedObject(items) => items,
                        }
                    })
                    .concat(),
            }),
            SecondLayerOutput::Set(output) => SetMapping::SingleValue(SetMappingValue::Set(
                SetOperation::from_second_layer(output.as_ref(), step, map_id_supplier, tables),
            )),
            SecondLayerOutput::Primitive(output) => {
                SetMapping::SingleValue(SetMappingValue::Value(
                    ValueOperation::from_second_layer_output(output, step, map_id_supplier, tables),
                ))
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum SetTwoSetOperator {
    Union,
    Intersect,
    Without,
    SubtractedFrom,
    DisjointWith,
    DisjunctiveUnion,
}
#[derive(Debug, Clone, Copy)]
pub enum MultiSetOperator {
    Union,
    Intersect,
}
