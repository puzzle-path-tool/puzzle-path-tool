use std::fmt::Debug;

use itertools::Itertools;

use crate::layers::id_helpers::{EnumTableId, FieldId, PathString, StepId, TableId, WrappedId};
use crate::layers::second_layer::tables::{Field, TableBundle};

pub(crate) mod test {
    use super::{BoolCombinator, BooleanOutput, LogicStep, MathOperator, NumberComparor, NumberOutput, ObjectOutput, Output, SetObject, SetOutput, StepId, StepObject, TableBundle};

    pub(crate) fn build_example_step(tables: &TableBundle) -> LogicStep {
        LogicStep {
            name: "ExampleStep".to_string(),
            id: StepId::new(),
            description: "ExampleStepDescritpion".to_string(),
            step_objects: {
                let mut items = vec![];
                if let Some(deduction) = tables.get_deduction_tables().first() {
                    items.push(StepObject::DeductionObject {
                        id: 0,
                        table: deduction.get_id(),
                        in_pool: true,
                        emmit_or_consum: true,
                    });
                    items.push(StepObject::DeductionObject {
                        id: 1,
                        table: deduction.get_id(),
                        in_pool: false,
                        emmit_or_consum: true,
                    });
                }
                items
            },
            step_sets: {
                let mut items = vec![];
                if let Some(deduction) = tables.get_deduction_tables().first() {
                    items.push(SetObject::SetOfObjects(StepObject::DeductionObject {
                        id: 2,
                        table: deduction.get_id(),
                        in_pool: true,
                        emmit_or_consum: true,
                    }));
                }
                items
            },
            match_statement: BooleanOutput::BoolCombination {
                first: Box::new(BooleanOutput::NumberComparison {
                    first: Box::new(NumberOutput::Number { value: 5 }),
                    second: Box::new(NumberOutput::MathOperation {
                        first: Box::new(NumberOutput::Number { value: 6 }),
                        second: Box::new(NumberOutput::SetSize {
                            set: Box::new(SetOutput::SetObject { set_object_id: 2 }),
                        }),
                        operator: MathOperator::Div,
                    }),
                    operator: NumberComparor::Unequal,
                }),
                second: Box::new(BooleanOutput::BoolCombination {
                    first: Box::new(BooleanOutput::ObjectComparison {
                        first: Box::new(ObjectOutput::StepObject {
                            object_id: 0,
                            partial: None,
                        }),
                        second: Box::new(ObjectOutput::StepObject {
                            object_id: 1,
                            partial: None,
                        }),
                        equal: true,
                    }),
                    second: Box::new(BooleanOutput::ElementOfSet {
                        element: Box::new(Output::Object(Box::new(ObjectOutput::StepObject {
                            object_id: 0,
                            partial: None,
                        }))),
                        set: Box::new(SetOutput::SetObject { set_object_id: 2 }),
                    }),
                    operator: BoolCombinator::XOr,
                }),
                operator: BoolCombinator::And,
            },
        }
    }
}

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
    /*pub(crate) fn new(name: String, description: String, tables: &TableBundle) -> LogicStep {
        LogicStep { name, id: StepId::new(), description, step_objects: vec![], step_sets: vec![], match_statement: BooleanOutput::Boolean { value: true } }
    }*/
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
    pub(crate) fn get_setp_object_by_id(&self, object_id: usize) -> Option<&StepObject> {
        self.step_objects
            .iter()
            .find(|item| item.get_id() == object_id)
    }
    pub(crate) fn get_step_set_by_id(&self, object_id: usize) -> Option<&SetObject> {
        self.step_sets
            .iter()
            .find(|item| item.get_id() == object_id)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StepObject {
    BuildObject {
        id: usize,
        fields: BuildObjectFields,
    },
    DeductionObject {
        id: usize,
        table: TableId,
        in_pool: bool,
        emmit_or_consum: bool,
    },
}
impl StepObject {
    pub(crate) fn get_object_fields(
        &self,
        partial: &Option<PathString>,
        tables: &super::tables::TableBundle,
    ) -> Vec<(FieldId, PathString)> {
        match self {
            StepObject::DeductionObject {
                id: _,
                table,
                in_pool: _,
                emmit_or_consum: _,
            } => {
                if let Some(table) = tables.get_deduction_table_by_id(*table) {
                    table.table_fields(tables, partial)
                } else {
                    panic!("Deduction not in pool")
                }
            }
            StepObject::BuildObject { id: _, fields } => fields.object_fields(partial),
        }
    }
    pub(crate) fn get_id(&self) -> usize {
        match self {
            StepObject::BuildObject { id, fields: _ }
            | StepObject::DeductionObject {
                id,
                table: _,
                in_pool: _,
                emmit_or_consum: _,
            } => *id,
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct BuildObjectFields {
    value_fields: Vec<Field>,
    array_fields: Vec<(TableId, BuildObjectFields)>,
}
impl BuildObjectFields {
    /*pub(crate) fn new(value_fields: TODO_ObjectFieldTypeStandIn, tables: TableBundle) -> BuildObjectFields {
        let (fields, arrays) = value_fields
        .iter()
        .fold((vec![], vec![]), |(mut field_acc, mut array_acc), (field_name, field_type)|{
            let (field, arrays) = Field::new(field_name, PathString::new(), TableId::new(), &mut IdSupplier::new(), field_type, tables.get_enum_tables());
            field_acc.push(field);
            let arrays = arrays.iter().map(Self::from_array_table).collect();
            array_acc.append(arrays);
            (field_acc, array_acc)
        });
        BuildObjectFields { value_fields: fields, array_fields: arrays }
    }
    fn from_array_table(array: &ArrayTable) -> (TableId, BuildObjectFields){
        (array.get_id(), BuildObjectFields{
            value_fields: vec![array.get_field().to_owned()],
            array_fields: todo!(),
        })
    }*/
    pub(crate) fn flat_type(
        &self,
    ) -> (
        Vec<(FieldId, PathString)>,
        Vec<(TableId, Vec<TableId>, Vec<(FieldId, PathString)>)>,
    ) {
        let fields = self.value_fields.iter().map(|item| item.flatten()).concat();
        let arrays = self
            .array_fields
            .iter()
            .fold(vec![], |mut acc, (table_id, fields)| {
                let (current_fields, mut additonal_arrays) = fields.flat_type();
                acc.push((*table_id, vec![], current_fields));
                acc.append(
                    &mut additonal_arrays
                        .iter_mut()
                        .map(|(array_id, ref_ids, fields)| {
                            ref_ids.push(*table_id);
                            (*array_id, ref_ids.clone(), fields.clone())
                        })
                        .collect(),
                );
                acc
            });
        (fields, arrays)
    }
    pub(crate) fn object_fields(&self, partial: &Option<PathString>) -> Vec<(FieldId, PathString)> {
        let (fields, _) = self.flat_type();
        if let Some(partial) = partial {
            fields
                .iter()
                .filter_map(|(id, name)| {
                    if let Some(name) = name.out_of(partial) {
                        Some((*id, name))
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
pub(crate) enum SetObject {
    SetOfObjects(StepObject),
    SetOfSets(Box<SetObject>),
}
impl SetObject {
    pub(crate) fn get_id(&self) -> usize {
        match self {
            SetObject::SetOfObjects(step_object) => step_object.get_id(),
            SetObject::SetOfSets(set_object) => set_object.get_id(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum BooleanOutput {
    Boolean {
        value: bool,
    },
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
        element: Box<Output>,
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
        stand_in_id: WrappedId,
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
    Mapping {
        mapped_enum: Box<EnumOutput>,
        mapping_table: EnumTableId,
    },
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
        stand_in_id: WrappedId,
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
    Mapping {
        mapped_number: Box<NumberOutput>,
        mapping_table: EnumTableId,
    },
    Enum {
        enum_id: EnumTableId,
        value: String,
    },
    ObjectFieldEnum {
        object: Box<ObjectOutput>,
        field_id: usize,
    },
    MappingStandIn {
        stand_in_id: WrappedId,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectOutput {
    MappingStandIn {
        stand_in_id: WrappedId,
        partial: Option<PathString>,
        item_type: BuildObjectFields,
    },
    StepObject {
        object_id: usize,
        partial: Option<PathString>,
    },
    FixedObject {
        fields: Vec<(PathString, Option<FieldId>, Output)>,
    },
}
pub(crate) enum ObjectOutputField {
    Id(FieldId),
    PrimitiveValue(PrimitiveOutput),
    Set(SetOutput),
}

#[derive(Debug, Clone)]
pub(crate) enum SetOutput {
    MappedSet {
        set: Box<SetOutput>,
        stand_in_id: WrappedId,
        mapping: Output,
    },
    FilteredSet {
        set: Box<SetOutput>,
        stand_in_id: WrappedId,
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
        stand_in_id: WrappedId,
        item_type: BuildObjectFields,
        set_in_set_depth: i32,
    },
}
#[derive(Debug, Clone)]
pub(crate) enum Output {
    Object(Box<ObjectOutput>),
    Set(Box<SetOutput>),
    Primitive(PrimitiveOutput),
}
#[derive(Debug, Clone)]
pub(crate) enum PrimitiveOutput {
    Number(Box<NumberOutput>),
    Boolean(Box<BooleanOutput>),
    Enum(Box<EnumOutput>),
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
