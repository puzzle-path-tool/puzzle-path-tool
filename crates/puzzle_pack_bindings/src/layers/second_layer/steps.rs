use std::fmt::Debug;

use crate::layers::second_layer::{StepId, TableId, tables::Field};

#[derive(Debug, Clone)]
pub(super) struct LogicStep {
    name: String,
    id: StepId,
    description: String,
    step_objects: Vec<Box<StepObject>>,
    step_sets: Vec<Box<SetObject>>,
    match_statement: BooleanOutput,
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
        fields: Vec<Field>,
    },
}
#[derive(Debug, Clone)]
enum SetObject {
    SetOfObjects(Box<StepObject>),
    SetOfSets(Box<SetObject>),
}

#[derive(Debug, Clone)]
enum BooleanOutput {
    BoolCombination {
        first: Box<BooleanOutput>,
        second: Box<BooleanOutput>,
        operator: BoolCombinator,
    },
    SetComparison {
        first: Box<SetOutput>,
        second: Box<SetOutput>,
    },
    ElementOfSet {
        element: Box<ObjectOutput>,
        set: Box<SetOutput>,
    },
    NumberComparison {
        first: Box<NumberOutput>,
        second: Box<NumberOutput>,
        operator: NumberComparor,
    },
    EnumComparison {
        first: Box<EnumOutput>,
        second: Box<EnumOutput>,
        equal: bool,
    },
    ObjectComparison {
        first: Box<ObjectOutput>,
        second: Box<ObjectOutput>,
        equal: bool,
    },
    ObjectFieldBoolean {
        object: Box<ObjectOutput>,
        field_id: usize,
    },
    MappingStandIn {
        stand_in_id: usize,
    },
}

#[derive(Debug, Clone, Copy)]
enum BoolCombinator {
    And,
    Or,
    XOr,
}

#[derive(Debug, Clone, Copy)]
enum SetComparor {
    SubsetOf,
    TrueSubsetOf,
    SupersetOf,
    TrueSupersetOf,
}

#[derive(Debug, Clone, Copy)]
enum NumberComparor {
    Equal,
    Unequal,
    Smaller,
    Bigger,
    SmallerEqual,
    BiggerEqual,
}

#[derive(Debug, Clone)]
enum NumberOutput {
    Mapping, //ToDo
    Number {
        value: i32,
    },
    MathOperation {
        first: Box<NumberOutput>,
        second: Box<NumberOutput>,
        operator: MathOperator,
    },
    SetSize {
        set: Box<SetOutput>,
    },
    ObjectFieldNumber {
        object: Box<ObjectOutput>,
        field_id: usize,
    },
    MappingStandIn {
        stand_in_id: usize,
    },
}

#[derive(Debug, Clone, Copy)]
enum MathOperator {
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Rem,
    Expon,
}

#[derive(Debug, Clone)]
enum EnumOutput {
    Mapping, //ToDo
    Enum {
        value: String,
    },
    ObjectFieldEnum {
        object: Box<ObjectOutput>,
        field_id: usize,
    },
    MappingStandIn {
        stand_in_id: usize,
    },
}

#[derive(Debug, Clone)]
struct ObjectOutput {
    object_id: usize,
    partial: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
enum SetOutput {
    MappedSet {
        set: Box<SetOutput>,
        stand_in_id: usize,
        mapping: SetMapping,
    },
    FilteredSet {
        set: Box<SetOutput>,
        stand_in_id: usize,
        filter: BooleanOutput,
    },
    SetObject {
        set_object_id: usize,
    },
    TwoSetOperation {
        first: Box<SetOutput>,
        second: Box<SetOutput>,
        operator: TwoSetOperator,
    },
    MultiSetOperation {
        set: Box<SetOutput>,
        operator: MultiSetOperator,
    },
    MappingStandIn {
        stand_in_id: usize,
    },
}
#[derive(Debug, Clone)]
enum SetMapping {
    NumberOp { output: Box<NumberOutput> },
    BoolOp { output: Box<BooleanOutput> },
    EnumOp { output: Box<EnumOutput> },
    ObjectOp { output: Box<ObjectOutput> },
    SetOp { output: Box<SetOutput> },
}
#[derive(Debug, Clone, Copy)]
enum TwoSetOperator {
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
