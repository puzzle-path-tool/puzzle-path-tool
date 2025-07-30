use crate::layers::second_layer::{
    self, StepId, TableId,
    steps::{ObjectOutput, TwoSetOperator},
};

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
        IdSupplier { current: 0, conversion: vec![] }
    }
    fn next(&mut self) -> usize {
        let result = self.current;
        self.current += 1;
        result
    }
    fn convert(&mut self, old_id: usize) -> usize {
        if let Some((_, new_id)) = self.conversion.iter().find(|item|{item.0 == old_id}) {
            *new_id
        } else {
            let new_id = self.next();
            self.conversion.push((old_id, new_id));
            new_id
        }
    }
}

pub struct LogicStep {
    id: StepId,
    match_statement: ValueOperation,
    step_objects: Vec<StepObject>,
    step_sets: Vec<SetObject>,
}
impl LogicStep {
    fn from_second_layer(logic_step: &second_layer::steps::LogicStep) -> LogicStep {
        LogicStep {
            id: logic_step.get_id(),
            match_statement: ValueOperation::from_second_layer_bool_output(
                logic_step.get_match_statement(),
                logic_step,
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
    value_fields: Vec<(usize, Option<TableId>)>,
    array_fields: Vec<(TableId, Vec<(usize, Option<TableId>)>)>,
}
impl BuildObjectFields {
    fn from_second_layer(fields: &second_layer::steps::BuildObjectFields) -> BuildObjectFields {
        let (value_fields, array_fields) = fields.flatten();
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
        stand_in_id: usize,
        field_id: usize,
    },
    Mapping, //ToDo
}
impl ValueOperation {
    fn from_second_layer_bool_output(
        bool_output: &second_layer::steps::BooleanOutput,
        step: &second_layer::steps::LogicStep,
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
                )),
                second: Box::new(ValueOperation::from_second_layer_bool_output(
                    second.as_ref(),
                    step,
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
                first: Box::new(SetOperation::from_second_layer(first.as_ref(), step)),
                second: Box::new(SetOperation::from_second_layer(second.as_ref(), step)),
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
                ValueOperation::TwoValueOp {
                    first: Box::new(ValueOperation::SetSize { set: SetOperation::MappedSet { set: (), stand_in_id: (), mapping: (), filter: () } }),
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
                )),
                second: Box::new(ValueOperation::from_second_layer_number_output(
                    second.as_ref(),
                    step,
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
                )),
                second: Box::new(ValueOperation::from_second_layer_enum_output(
                    second.as_ref(),
                    step,
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
                ValueOperation::field_value_from_second_layer(object, field_id, step)
            }
            second_layer::steps::BooleanOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: *stand_in_id,
                }
            }
        }
    }
    fn from_second_layer_number_output(
        number_output: &second_layer::steps::NumberOutput,
        step: &second_layer::steps::LogicStep,
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
                )),
                second: Box::new(ValueOperation::from_second_layer_number_output(
                    second.as_ref(),
                    step,
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
                set: Box::new(SetOperation::from_second_layer(set.as_ref(), step)),
            },
            second_layer::steps::NumberOutput::ObjectFieldNumber { object, field_id } => {
                ValueOperation::field_value_from_second_layer(object, field_id, step)
            }
            second_layer::steps::NumberOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: *stand_in_id,
                }
            }
        }
    }
    fn from_second_layer_enum_output(
        enum_output: &second_layer::steps::EnumOutput,
        step: &second_layer::steps::LogicStep,
    ) -> ValueOperation {
        match enum_output {
            second_layer::steps::EnumOutput::Mapping => todo!(),
            second_layer::steps::EnumOutput::Enum { value } => {
                ValueOperation::FixedValue { value: todo!() }
            }
            second_layer::steps::EnumOutput::ObjectFieldEnum { object, field_id } => {
                ValueOperation::field_value_from_second_layer(object, field_id, step)
            }
            second_layer::steps::EnumOutput::MappingStandIn { stand_in_id } => {
                ValueOperation::MappingStandIn {
                    mapping_id: *stand_in_id,
                }
            }
        }
    }
    fn field_value_from_second_layer(
        object: &Box<second_layer::steps::ObjectOutput>,
        field_id: &usize,
        step: &second_layer::steps::LogicStep,
    ) -> ValueOperation {
        let object = object.as_ref();

        ValueOperation::FieldValue {
            object_id: todo!(),
            field_id: *field_id,
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
        stand_in_id: usize,
    },
    MappingStandInFieldSet {
        stand_in_id: usize,
        field_id: usize,
    },
}
impl SetOperation {
    fn from_second_layer(
        set_output: &second_layer::steps::SetOutput,
        step: &second_layer::steps::LogicStep,
    ) -> SetOperation {
        todo!()
    }
}
#[derive(Debug, Clone)]
enum SetMapping {
    Value(ValueOperation),
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
