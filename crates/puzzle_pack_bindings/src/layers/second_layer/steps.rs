use std::fmt::Debug;

use crate::layers::second_layer::{StepId, TableId, tables::Field};

#[derive(Debug, Clone)]
pub(crate) struct LogicStep {
    name: String,
    id: StepId,
    description: String,
    step_objects: Vec<StepObject>,
    step_sets: Vec<SetObject>,
    match_statement: BooleanOutput,
}
impl LogicStep {
    pub(crate) fn get_id(&self) -> StepId {
        self.id
    }
    pub(crate) fn get_step_objects(&self) -> Vec<&StepObject> {
        self.step_objects.iter().map(|item| item).collect()
    }
    pub(crate) fn get_step_sets(&self) -> Vec<&SetObject> {
        self.step_sets.iter().map(|item| item).collect()
    }
    pub(crate) fn get_match_statement(&self) -> &BooleanOutput {
        &self.match_statement
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StepObject {
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
#[derive(Debug, Clone)]
pub(crate) struct BuildObjectFields {
    value_fields: Vec<Field>,
    array_fields: Vec<(TableId, BuildObjectFields)>,
}
impl BuildObjectFields {
    pub(crate) fn flatten(
        &self,
    ) -> (
        Vec<(usize, Option<TableId>)>,
        Vec<(TableId, Vec<(usize, Option<TableId>)>)>,
    ) {
        let (field_ids, array_ids) = self.value_fields.iter().fold(
            (vec![], vec![]),
            |(mut acc_ids, mut acc_arrays), item| {
                let (mut current_ids, mut current_arrays) = item.flatten();
                acc_ids.append(&mut current_ids);
                acc_arrays.append(&mut current_arrays);
                (acc_ids, acc_arrays)
            },
        );
        let mut max_id = field_ids
            .iter()
            .fold(0, |x, item| if x < *item { *item } else { x });
        let mut fields: Vec<(usize, Option<TableId>)> =
            field_ids.iter().map(|item| (*item, None)).collect();
        let mut arrays: Vec<(TableId, Vec<(usize, Option<TableId>)>)> = vec![];
        for array_id in array_ids {
            max_id += 1;
            fields.push((max_id, Some(array_id)));
            if let Some(current) = self
                .array_fields
                .iter()
                .find(|item: &&(TableId, BuildObjectFields)| item.0 == array_id)
            {
                let (fields_of_array, mut arrayfields_of_array) = current.1.flatten();
                arrays.push((array_id, fields_of_array));
                arrays.append(&mut arrayfields_of_array);
            } else {
                panic!("BuildObjectFields missing array of id: {array_id:?}")
            }
        }
        (fields, arrays)
    }
}
#[derive(Debug, Clone)]
pub(crate) enum SetObject {
    SetOfObjects(StepObject),
    SetOfSets(Box<SetObject>),
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanOutput {
    BoolCombination {
        first: Box<BooleanOutput>,
        second: Box<BooleanOutput>,
        operator: BoolCombinator,
    },
    SetComparison {
        first: Box<SetOutput>,
        second: Box<SetOutput>,
        operator: SetComparor,
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
pub(crate) enum BoolCombinator {
    And,
    Or,
    XOr,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SetComparor {
    Equal,
    SubsetOf,
    TrueSubsetOf,
    SupersetOf,
    TrueSupersetOf,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum NumberComparor {
    Equal,
    Unequal,
    Smaller,
    Bigger,
    SmallerEqual,
    BiggerEqual,
}

#[derive(Debug, Clone)]
pub(crate) enum NumberOutput {
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
pub(crate) enum MathOperator {
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Rem,
    Expon,
}

#[derive(Debug, Clone)]
pub(crate) enum EnumOutput {
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
pub(crate) struct ObjectOutput {
    object_id: usize,
    partial: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub(crate) enum SetOutput {
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
pub(crate) enum SetMapping {
    NumberOp { output: Box<NumberOutput> },
    BoolOp { output: Box<BooleanOutput> },
    EnumOp { output: Box<EnumOutput> },
    ObjectOp { output: Box<ObjectOutput> },
    SetOp { output: Box<SetOutput> },
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum TwoSetOperator {
    Union,
    Intersect,
    Without,
    SubtractedFrom,
    DisjointWith,
    DisjunctiveUnion,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum MultiSetOperator {
    Union,
    Intersect,
}
