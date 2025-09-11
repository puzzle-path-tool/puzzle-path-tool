#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub struct MatchExpression {
    value: MatchValue,
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum MatchValue {
    Variable { id: i32 },
    Constant(serde_json::Value),
    Operation(Box<MatchOperation>),
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum MatchOperation {
    Enum(EnumOperation),
    Bool(BoolOperation),
    Int(IntOperation),
    Set(SetOperation),
    Obj(ObjOperation),
    Table(TableOperation),
    Quantor(QuantorOperation),
    Pool(PoolOperation),
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum EnumOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum BoolOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum IntOperation {
    Cmp {
        a: MatchExpression,
        b: MatchExpression,
        o: IntCmpOp,
    },
    Math {
        a: MatchExpression,
        b: MatchExpression,
        o: IntMathOp,
    },
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum IntCmpOp {
    Equals,
    NotEquals,
    SmallerThanOrEqual,
    BiggerThanOrEqual,
    SmallerThan,
    BiggerThan,
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum IntMathOp {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum SetOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum ObjOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum TableOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum QuantorOperation {}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum PoolOperation {}
