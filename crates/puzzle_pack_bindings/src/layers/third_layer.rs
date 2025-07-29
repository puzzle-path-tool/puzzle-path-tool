use crate::layers::second_layer::{self, StepId, TableId};

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
            match_statement: todo!(),
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
    value_fields: Vec<(usize)>,
    array_fields: Vec<(usize, BuildObjectFields)>,
}
impl BuildObjectFields {
    fn from_second_layer(fields: &Vec<second_layer::tables::Field>) -> BuildObjectFields {
        todo!()
    }
}
#[derive(Debug, Clone)]
enum SetObject {
    SetOfObjects(StepObject),
    SetOfSets(Box<SetObject>),
}
impl SetObject {
    fn from_second_layer(set: &second_layer::steps::SetObject) -> SetObject {
        todo!()
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
