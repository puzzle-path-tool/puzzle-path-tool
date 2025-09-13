use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::{TYPES, identifier::Identifier};

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "RegisterVariablePuzzptApi")]
pub struct RegisterVariable {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "fieldType")]
    field_type: FieldType,
}

#[distributed_slice(TYPES)]
static REGISTER_VARIABLE_TYPE: fn() -> String = RegisterVariable::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "EmitExpressionPuzzptApi")]
pub struct EmitExpression {
    #[serde(rename = "deduction")]
    deduction: Identifier,
    #[serde(rename = "expression")]
    expression: MatchExpression,
}

#[distributed_slice(TYPES)]
static EMIT_EXPRESSION_TYPE: fn() -> String = EmitExpression::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "FieldTypePuzzptApi")]
pub enum FieldType {
    #[serde(rename = "Int")]
    Int,
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "Enum")]
    Enum(Vec<String>),
    #[serde(rename = "Set")]
    Set(Box<FieldType>),
    #[serde(rename = "Obj")]
    Obj(HashMap<String, FieldType>),
}

#[distributed_slice(TYPES)]
static FIELD_TYPE_TYPE: fn() -> String = FieldType::decl;

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
#[serde(rename = "BoolOperationPuzzptApi")]
pub enum BoolOperation {
    #[serde(rename = "Cmp")]
    Cmp {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: BoolCmpOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Logic")]
    Logic {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: BoolLogicOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Not")]
    Not {
        #[serde(rename = "a")]
        a: MatchExpression,
    },
    #[serde(rename = "Fold")]
    Fold {
        #[serde(rename = "o")]
        o: BoolFoldOp,
        #[serde(rename = "a")]
        a: MatchExpression,
    },
    #[serde(rename = "All")]
    All {
        #[serde(rename = "o")]
        o: BoolAllOp,
        #[serde(rename = "a")]
        a: MatchExpression,
    },
    #[serde(rename = "None")]
    None {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "And")]
    And {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Or")]
    Or {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
}

#[distributed_slice(TYPES)]
static BOOL_OPERATION_TYPE: fn() -> String = BoolOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "BoolCmpOpPuzzptApi")]
pub enum BoolCmpOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
}

#[distributed_slice(TYPES)]
static BOOL_CMP_OP_TYPE: fn() -> String = BoolCmpOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "BoolLogicOpPuzzptApi")]
pub enum BoolLogicOp {
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "xor")]
    Xor,
    #[serde(rename = "nor")]
    Nor,
    #[serde(rename = "nand")]
    Nand,
    #[serde(rename = "xnor")]
    Xnor,
}

#[distributed_slice(TYPES)]
static BOOL_LOGIC_OP_TYPE: fn() -> String = BoolLogicOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "BoolFoldOpPuzzptApi")]
pub enum BoolFoldOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "xor")]
    Xor,
    #[serde(rename = "nor")]
    Nor,
    #[serde(rename = "nand")]
    Nand,
    #[serde(rename = "xnor")]
    Xnor,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "none")]
    None,
}

#[distributed_slice(TYPES)]
static BOOL_FOLD_OP_TYPE: fn() -> String = BoolFoldOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "BoolAllOpPuzzptApi")]
pub enum BoolAllOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

#[distributed_slice(TYPES)]
static BOOL_ALL_OP_TYPE: fn() -> String = BoolAllOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "IntOperationPuzzptApi")]
pub enum IntOperation {
    #[serde(rename = "Cmp")]
    Cmp {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: IntCmpOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Math")]
    Math {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: IntMathOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Fold")]
    Fold {
        #[serde(rename = "o")]
        o: IntFoldOp,
        #[serde(rename = "a")]
        a: MatchExpression,
    },
    #[serde(rename = "Sum")]
    Sum {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Product")]
    Product {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Min")]
    Min {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Max")]
    Max {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "All")]
    All {
        #[serde(rename = "o")]
        o: IntAllOp,
        #[serde(rename = "items")]
        items: MatchExpression,
    },
    #[serde(rename = "IsPrime")]
    IsPrime {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
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
    #[serde(rename = "Cmp")]
    Cmp {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: SetCmpOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "ElementOf")]
    ElementOf {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Contains")]
    Contains {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Join")]
    Join {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: SetJoinOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "Fold")]
    Fold {
        #[serde(rename = "o")]
        o: SetFoldOp,
        #[serde(rename = "items")]
        items: MatchExpression,
    },
    #[serde(rename = "Union")]
    Union {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Intersect")]
    Intersect {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "DisjunctiveUnion")]
    DisjunctiveUnion {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "All")]
    All {
        #[serde(rename = "o")]
        o: SetAllOp,
        #[serde(rename = "items")]
        items: MatchExpression,
    },
    #[serde(rename = "AllDisjoint")]
    AllDisjoint {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "Size")]
    Size {
        #[serde(rename = "item")]
        item: MatchExpression,
    },
    #[serde(rename = "Map")]
    Map {
        #[serde(rename = "item")]
        item: MatchExpression,
        #[serde(rename = "f")]
        f: (), //TODO
    },
}

#[distributed_slice(TYPES)]
static SET_OPERATION_TYPE: fn() -> String = SetOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "SetCmpOpPuzzptApi")]
pub enum SetCmpOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
    #[serde(rename = "subset of")]
    SubsetOf,
    #[serde(rename = "superset of")]
    SupersetOf,
    #[serde(rename = "true subset of")]
    TrueSubsetOf,
    #[serde(rename = "true superset of")]
    TrueSupersetOf,
    #[serde(rename = "disjoint with")]
    DisjointWith,
}

#[distributed_slice(TYPES)]
static SET_CMP_OP_TYPE: fn() -> String = SetCmpOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "SetJoinOpPuzzptApi")]
pub enum SetJoinOp {
    #[serde(rename = "union")]
    Union,
    #[serde(rename = "intersect")]
    Intersect,
    #[serde(rename = "without")]
    Without,
    #[serde(rename = "subtracted from")]
    SubtractedFrom,
    #[serde(rename = "disjunctive union")]
    DisjunctiveUnion,
}

#[distributed_slice(TYPES)]
static SET_JOIN_OP_TYPE: fn() -> String = SetJoinOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "SetFoldOpPuzzptApi")]
pub enum SetFoldOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "union")]
    Union,
    #[serde(rename = "intersect")]
    Intersect,
    #[serde(rename = "disjunctive union")]
    DisjunctiveUnion,
}

#[distributed_slice(TYPES)]
static SET_FOLD_OP_TYPE: fn() -> String = SetFoldOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "SetAllOpPuzzptApi")]
pub enum SetAllOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "disjoint")]
    Disjoint,
}

#[distributed_slice(TYPES)]
static SET_ALL_OP_TYPE: fn() -> String = SetAllOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "ObjOperationPuzzptApi")]
pub enum ObjOperation {
    #[serde(rename = "Cmp")]
    Cmp {
        #[serde(rename = "a")]
        a: MatchExpression,
        #[serde(rename = "o")]
        o: ObjCmpOp,
        #[serde(rename = "b")]
        b: MatchExpression,
    },
    #[serde(rename = "All")]
    All {
        #[serde(rename = "o")]
        o: ObjAllOp,
        #[serde(rename = "items")]
        items: MatchExpression,
    },
    #[serde(rename = "Equal")]
    Equal {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
    #[serde(rename = "NoneEqual")]
    NoneEqual {
        #[serde(rename = "items")]
        items: Vec<MatchExpression>,
    },
}

#[distributed_slice(TYPES)]
static OBJ_OPERATION_TYPE: fn() -> String = ObjOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "ObjCmpOpPuzzptApi")]
pub enum ObjCmpOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
}

#[distributed_slice(TYPES)]
static OBJ_CMP_OP_TYPE: fn() -> String = ObjCmpOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "ObjAllOpPuzzptApi")]
pub enum ObjAllOp {
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = "!=")]
    NotEquals,
}

#[distributed_slice(TYPES)]
static OBJ_ALL_OP_TYPE: fn() -> String = ObjAllOp::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "TableOperationPuzzptApi")]
pub enum TableOperation {
    #[serde(rename = "Forwards")]
    Forwards {
        #[serde(rename = "fieldTypeA")]
        field_type_a: FieldType,
        #[serde(rename = "fieldTypeB")]
        field_type_b: FieldType,
        #[serde(rename = "mapping")]
        mapping: Vec<()>, //TODO
        #[serde(rename = "item")]
        item: MatchExpression,
    },
    #[serde(rename = "Backwards")]
    Backwards {
        #[serde(rename = "fieldTypeA")]
        field_type_a: FieldType,
        #[serde(rename = "fieldTypeB")]
        field_type_b: FieldType,
        #[serde(rename = "mapping")]
        mapping: Vec<()>, //TODO
        #[serde(rename = "item")]
        item: MatchExpression,
    },
}

#[distributed_slice(TYPES)]
static TABLE_OPERATION_TYPE: fn() -> String = TableOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "QuantorOperationPuzzptApi")]
pub enum QuantorOperation {
    #[serde(rename = "All")]
    All {
        #[serde(rename = "variables")]
        variables: Vec<RegisterVariable>,
        #[serde(rename = "requirements")]
        requirements: Vec<MatchExpression>,
        #[serde(rename = "where_clauses")]
        where_clauses: Vec<MatchExpression>,
    },
    #[serde(rename = "Exists")]
    Exists {
        #[serde(rename = "variables")]
        variables: Vec<RegisterVariable>,
        #[serde(rename = "requirements")]
        requirements: Vec<MatchExpression>,
        #[serde(rename = "where_clauses")]
        where_clauses: Vec<MatchExpression>,
    },
}

#[distributed_slice(TYPES)]
static QUANTOR_OPERATION_TYPE: fn() -> String = QuantorOperation::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "PoolOperationPuzzptApi")]
pub enum PoolOperation {
    #[serde(rename = "One")]
    One {
        #[serde(rename = "item")]
        item: MatchExpression,
        #[serde(rename = "deduction")]
        deduction: Identifier,
    },
    #[serde(rename = "Many")]
    Many {
        #[serde(rename = "items")]
        items: MatchExpression,
        #[serde(rename = "deduction")]
        deduction: Identifier,
    },
}

#[distributed_slice(TYPES)]
static POOL_OPERATION_TYPE: fn() -> String = PoolOperation::decl;
