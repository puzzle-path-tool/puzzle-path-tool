extern crate puzzle_pack_bindings;
use std::rc::Rc;
use std::sync::Mutex;

use puzzle_pack_bindings::layers::id_helpers::{EnumTableId, TableId};
use puzzle_pack_bindings::layers::third_layer::steps::*;
use puzzle_pack_bindings::layers::third_layer::tables::*;

pub fn execute_logic_step(step: &LogicStep, pool: &mut TablePool, variation: usize) -> usize {
    let mut temp_pool = TempPool::new(step, pool);
    if let Some(variation) = executeValueOperation(step.get_match_statement(), pool, &mut temp_pool)
        .try_true(variation, &mut temp_pool)
    {
        temp_pool.emit_and_consumn(pool);
        variation
    } else {
        0
    }
}

fn executeValueOperation(
    operation: &ValueOperation,
    pool: &TablePool,
    temp_pool: &mut TempPool,
) -> FieldValue {
    match operation {
        ValueOperation::TwoValueOp {
            first,
            second,
            operator,
        } => FieldValue::two_value_op(
            executeValueOperation(first.as_ref(), pool, temp_pool),
            executeValueOperation(second.as_ref(), pool, temp_pool),
            *operator,
        ),
        ValueOperation::TwoSetOp {
            first,
            second,
            operator,
        } => FieldValue::two_set_op(
            execute_set_operation(first.as_ref(), pool, temp_pool),
            execute_set_operation(second.as_ref(), pool, temp_pool),
            *operator,
        ),
        ValueOperation::SetSize { set } => {
            execute_set_operation(set.as_ref(), pool, temp_pool).size()
        }
        ValueOperation::FixedValue { value } => FieldValue::fixed(*value, temp_pool),
        ValueOperation::FieldValue {
            object_id,
            field_id,
        } => temp_pool.object_field(*object_id, *field_id),
        ValueOperation::MappedValue { mapping_id } => temp_pool.mapped_value(*mapping_id),
        ValueOperation::MappedObjectFieldValue {
            mapping_id,
            field_id,
        } => temp_pool.mapped_object_field_value(*mapping_id, *field_id),
        ValueOperation::ConvertedValue {
            converted_value,
            look_up_table,
        } => {
            /*executeValueOperation(converted_value.as_ref(), pool, temp_pool)
            .convert(pool.get_conversion_table(*look_up_table))*/
            todo!()
        }
    }
}

fn execute_set_operation(
    operation: &SetOperation,
    pool: &TablePool,
    temp_pool: &mut TempPool,
) -> SetValue {
    todo!()
}

#[derive(Debug)]
struct TempPool {
    objects: Vec<TempObject>,
    sets: Vec<TempSet>,
    validity_handlers: Vec<Validity>,
}
impl TempPool {
    fn new(step: &LogicStep, pool: &TablePool) -> TempPool {
        let step_objects = step
            .get_step_objects()
            .iter()
            .map(|step_object| match step_object {
                StepObject::DeductionObject {
                    id,
                    table,
                    in_pool,
                    emmit_or_consum,
                } => {
                    let (fields, array_fields) = pool.get_deduction_type(*table);
                    TempObject {
                        id: *id,
                        fields,
                        array_fields,
                        deduction_info: Some(DeductionInfo {
                            table: *table,
                            in_pool: *in_pool,
                            emit_or_consumn: *emmit_or_consum,
                        }),
                    }
                }
                StepObject::BuildObject { id, fields } => TempObject {
                    id: *id,
                    fields: vec![(); fields.get_field_size()]
                        .iter()
                        .map(|_| FieldValue::new())
                        .collect(),
                    array_fields: fields
                        .get_array_fields()
                        .iter()
                        .map(|array_field| (array_field.get_id(), vec![]))
                        .collect(),
                    deduction_info: None,
                },
            })
            .collect();
        //todo: step.get_step_sets()
        let step_sets = todo!();
        TempPool {
            objects: step_objects,
            sets: step_sets,
            validity_handlers: vec![],
        }
    }
    fn object_field(&self, object_id: usize, field_id: usize) -> FieldValue {
        if let Some(value) = self
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .and_then(|object| Some(object.fields[field_id].clone()))
        {
            value
        } else {
            panic!()
        }
    }
    fn mapped_value(&self, mapping_id: usize) -> FieldValue {
        todo!()
    }
    fn mapped_object_field_value(&self, mapping_id: usize, field_id: usize) -> FieldValue {
        todo!()
    }
    fn emit_and_consumn(self, pool: &mut TablePool) {
        todo!()
    }
    fn is_valid(&self, validity_id: usize) -> bool {
        self.validity_handlers[validity_id].is_valid()
    }
    fn invalidate(&mut self, validity_id: usize) {
        self.validity_handlers[validity_id].invalidate()
    }
    fn invalidate_was_valid(&mut self, validity_id: usize) -> bool {
        self.validity_handlers[validity_id].invalidate_was_valid()
    }
    fn new_validity(&mut self) -> usize {
        self.validity_handlers.push(Validity::new());
        self.validity_handlers.len() - 1
    }
}
#[derive(Debug, Clone, Copy)]
struct Validity {
    valid: bool,
}
impl Validity {
    fn is_valid(&self) -> bool {
        self.valid
    }
    fn invalidate_was_valid(&mut self) -> bool {
        let result = self.valid;
        self.valid = false;
        result
    }
    fn invalidate(&mut self) {
        self.valid = false;
    }
    fn new() -> Validity {
        Validity { valid: true }
    }
}

#[derive(Debug)]
struct TempObject {
    id: usize,
    fields: Vec<FieldValue>,
    array_fields: Vec<(TableId, Vec<(Vec<usize>, Vec<FieldValue>)>)>,
    deduction_info: Option<DeductionInfo>,
}

#[derive(Debug, Clone)]
struct FieldValue {
    value_range: Option<Vec<(i32, usize)>>,
}
impl FieldValue {
    fn new() -> FieldValue {
        FieldValue { value_range: None }
    }
    fn fixed(value: i32, temp_pool: &mut TempPool) -> FieldValue {
        FieldValue {
            value_range: Some(vec![(value, temp_pool.new_validity())]),
        }
    }
    /*fn convert(self, table: &LookUpTable) -> FieldValue {
        todo!()
    }*/
    fn two_value_op(
        first: FieldValue,
        second: FieldValue,
        operator: TwoValueOperator,
    ) -> FieldValue {
        todo!()
    }
    fn two_set_op(first: SetValue, second: SetValue, operator: ValueTwoSetOperator) -> FieldValue {
        todo!()
    }
    fn try_true(self, variation: usize, temp_pool: &mut TempPool) -> Option<usize> {
        if let Some(mut value_range) = self.value_range {
            let mut current: usize = 0;
            let valid_values = value_range.iter_mut().filter(|(value, valid)| {
                if *value != 1 {
                    temp_pool.invalidate(*valid);
                    false
                } else if current == variation && temp_pool.is_valid(*valid) {
                    current += 1;
                    true
                } else if temp_pool.invalidate_was_valid(*valid) {
                    current += 1;
                    false
                } else {
                    false
                }
            });
            Some(current)
        } else {
            todo!()
        }
    }
    /*fn get_connected(&self) -> FieldValue {
        //Todo
        if let Some(value_range) = &self.value_range {
            FieldValue {
                value_range: value_range
                    .iter()
                    .filter(|(_, valid)| {
                        **valid
                    })
                    .map(|(value, valid)| Some((*value, valid.clone())))
                    .collect(),
            }
        } else {
            todo!()
        }
    }*/
}
#[derive(Debug, Clone, Copy)]
struct DeductionInfo {
    table: TableId,
    in_pool: bool,
    emit_or_consumn: bool,
}

#[derive(Debug, Clone)]
struct TempSet {}

#[derive(Debug, Clone)]
enum SetValue {}
impl SetValue {
    fn size(&self) -> FieldValue {
        todo!()
    }
    fn two_set_op(first: SetValue, second: SetValue, operator: SetTwoSetOperator) -> SetValue {
        todo!()
    }
}

pub struct TablePool {}

impl TablePool {
    fn get_deduction_type(
        &self,
        id: TableId,
    ) -> (
        Vec<FieldValue>,
        Vec<(TableId, Vec<(Vec<usize>, Vec<FieldValue>)>)>,
    ) {
        todo!()
    }
    /*fn get_conversion_table(&self, table_id: EnumTableId) -> &LookUpTable {
        todo!()
    }*/
}
