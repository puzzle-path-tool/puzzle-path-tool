use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::TYPES;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "MatchExpressionPuzzptApi")]
pub struct MatchExpression {
    #[serde(rename = "value")]
    value: MatchValue,
}

#[distributed_slice(TYPES)]
static MATCH_EXPRESSION_TYPE: fn() -> String = MatchExpression::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "MatchValuePuzzptApi")]
pub enum MatchValue {
    #[serde(rename = "Variable")]
    Variable { id: i32 },
    #[serde(rename = "Constant")]
    Constant(Box<MatchConstant>),
    #[serde(rename = "Operation")]
    Operation(Box<MatchOperation>),
}

#[distributed_slice(TYPES)]
static MATCH_VALUE_TYPE: fn() -> String = MatchValue::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "MatchConstantPuzzptApi")]
pub enum MatchConstant {
    #[serde(rename = "Enum")]
    Enum(String),
    #[serde(rename = "Bool")]
    Bool(bool),
    #[serde(rename = "Int")]
    Int(i32),
    #[serde(rename = "Set")]
    Set(Vec<MatchExpression>),
    #[serde(rename = "Obj")]
    Obj(HashMap<String, MatchExpression>),
}

#[distributed_slice(TYPES)]
static MATCH_CONSTANT_TYPE: fn() -> String = MatchConstant::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "MatchOperationPuzzptApi")]
pub enum MatchOperation {
    #[serde(rename = "Enum")]
    Enum(EnumOperation),
    #[serde(rename = "Bool")]
    Bool(BoolOperation),
    #[serde(rename = "Int")]
    Int(IntOperation),
    #[serde(rename = "Set")]
    Set(SetOperation),
    #[serde(rename = "Obj")]
    Obj(ObjOperation),
    #[serde(rename = "Table")]
    Table(TableOperation),
    #[serde(rename = "Quantor")]
    Quantor(QuantorOperation),
    #[serde(rename = "Pool")]
    Pool(PoolOperation),
}

#[distributed_slice(TYPES)]
static MATCH_OPERATION_TYPE: fn() -> String = MatchOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "EnumOperationPuzzptApi")]
pub enum EnumOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static ENUM_OPERATION_TYPE: fn() -> String = EnumOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "BoolOperationPuzzptApi")]
pub enum BoolOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static BOOL_OPERATION_TYPE: fn() -> String = BoolOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntOperationPuzzptApi")]
pub enum IntOperation {
    #[serde(rename = "Cmp")]
    Cmp {
        a: MatchExpression,
        o: IntCmpOp,
        b: MatchExpression,
    },
    #[serde(rename = "Math")]
    Math {
        a: MatchExpression,
        o: IntMathOp,
        b: MatchExpression,
    },
    #[serde(rename = "Fold")]
    Fold { o: IntFoldOp, a: MatchExpression },
    #[serde(rename = "Sum")]
    Sum { items: Vec<MatchExpression> },
    #[serde(rename = "Product")]
    Product { items: Vec<MatchExpression> },
    #[serde(rename = "Min")]
    Min { items: Vec<MatchExpression> },
    #[serde(rename = "Max")]
    Max { items: Vec<MatchExpression> },
    #[serde(rename = "All")]
    All { o: IntAllOp, items: MatchExpression },
    #[serde(rename = "IsPrime")]
    IsPrime { items: Vec<MatchExpression> },
}

#[distributed_slice(TYPES)]
static INT_OPERATION_TYPE: fn() -> String = IntOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntCmpOpPuzzptApi")]
pub enum IntCmpOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">")]
    GreaterThan,
}

#[distributed_slice(TYPES)]
static INT_CMP_OP_TYPE: fn() -> String = IntCmpOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntMathOpPuzzptApi")]
pub enum IntMathOp {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Subtract,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "//")]
    Divide,
    #[serde(rename = "mod")]
    Modulo,
    #[serde(rename = "rem")]
    Remainder,
    #[serde(rename = "**")]
    Power,
}

#[distributed_slice(TYPES)]
static INT_MATH_OP_TYPE: fn() -> String = IntMathOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntFoldOpPuzzptApi")]
pub enum IntFoldOp {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
}

#[distributed_slice(TYPES)]
static INT_FOLD_OP_TYPE: fn() -> String = IntFoldOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntAllOpPuzzptApi")]
pub enum IntAllOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "== 0")]
    EqualZero,
    #[serde(rename = "!= 0")]
    NotEqualZero,
    #[serde(rename = "<= 0")]
    LessThanOrEqualZero,
    #[serde(rename = ">= 0")]
    GreaterThanOrEqualZero,
    #[serde(rename = "< 0")]
    LessThanZero,
    #[serde(rename = "> 0")]
    GreaterThanZero,
    #[serde(rename = "prime")]
    Prime,
}

#[distributed_slice(TYPES)]
static INT_ALL_OP_TYPE: fn() -> String = IntAllOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "SetOperationPuzzptApi")]
pub enum SetOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static SET_OPERATION_TYPE: fn() -> String = SetOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "ObjOperationPuzzptApi")]
pub enum ObjOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static OBJ_OPERATION_TYPE: fn() -> String = ObjOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "TableOperationPuzzptApi")]
pub enum TableOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static TABLE_OPERATION_TYPE: fn() -> String = TableOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "QuantorOperationPuzzptApi")]
pub enum QuantorOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static QUANTOR_OPERATION_TYPE: fn() -> String = QuantorOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "PoolOperationPuzzptApi")]
pub enum PoolOperation {
    Placeholder,
}

#[distributed_slice(TYPES)]
static POOL_OPERATION_TYPE: fn() -> String = PoolOperation::decl;

