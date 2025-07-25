use crate::layers::second_layer::tables::DeductionTable;



pub(super) struct LogicStep<'a> {
    name: String,
    description: String,
    emmited: Vec<TableObject<'a>>,
    consumed: Vec<TableObject<'a>>,
    match_statement: BooleanOutput,
}


struct TableObject<'a> {
    id: usize,
    table: &'a DeductionTable,
    is_array: bool,
}

enum BooleanOutput {
    BoolCombination { first: Box<BooleanOutput>, second: Box<BooleanOutput>, operatior: BoolCombinator },
    SetComparison {  },
    ElementOfSet {  },
    NumberComparison {  },
    ObjectComparison {  },
    ElementInPool {  },
    ObjectFieldBoolean {  },
}
enum BoolCombinator { And, Or, XOr }
enum SetComparor { SubsetOf, TrueSubsetOf, SupersetOf, TrueSupersetOf }
enum NumberComparor { Equal, Unequal, Smaller, Bigger, SmallerEqual, BiggerEqual}

enum NumberOutput {
    Number {  },
    MathOperation {  },
    SetSize {  },
    ObjectFieldNumber {  },
}

enum ObjectOutput {

}

enum EnumOutput {

}

trait SetOutput {}

/*
enum NumberBooleanOperation {

}

trait BooleanOutput {}
trait NumberOutput {}
trait EnumOutput {}
trait ObjectOutput {}
trait SetOutput {}
 */
