import type { FullSet } from "packs/core/modules/classic/deductions";
import type { ExamplePuzzptApi } from "./puzzpt_api";

export function do_stuff(param: ExamplePuzzptApi): string {
    return "";
}

// Int
// Char [SingleChar]
// Bool [boolean]
// Enum [one of const string]
// Object<T> (Position) (many types, many fields)
// Array<T> (dyn lenth, one type)

// Enum (no values) [Int?]
// Position [Object?]

// Tagging System
// Int ("Up" -> 0, "Down" -> 1, "Left" -> 2, "Right" -> 3)
// "Up"|"Down"|"Left"|"Right" -> Int

const directions = ["UP", "DOWN", "LEFT", "RIGHT"] as const;

type SingleChar<T extends string> =
    T extends `${infer TFirstChar}${infer TRest}`
        ? TRest extends ""
            ? T & TFirstChar
            : never
        : never;

const v = "A" + "";

takeChar("e");
// takeChar(v)

const a = "A";
type X = SingleChar<typeof a>;

function takeChar<const T extends string>(char: SingleChar<T>) {}

type Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

type IntegerRec<
    N,
    TAcc extends string,
> = TAcc extends `${infer TFirstDigit}${infer TRest}`
    ? TFirstDigit extends Digit
        ? IntegerRec<N, TRest>
        : never
    : N;

type PositiveIntegerInternal<N extends number> = IntegerRec<N, `${N}`>;

type NonZeroPositiveInteger<N extends number> = N extends 0
    ? never
    : PositiveIntegerInternal<N>;

type PositiveInteger<N extends number> = NonZeroPositiveInteger<N> | 0;

type NegativeIntegerInternal<N extends number> = `${N}` extends `-${infer TInt}`
    ? IntegerRec<N, `${TInt}`>
    : never;

type NonZeroNegativeInteger<N extends number> = N extends 0
    ? never
    : NegativeIntegerInternal<N>;

type NegativeInteger<N extends number> = NonZeroNegativeInteger<N> | 0;

type Integer<N extends number> = PositiveInteger<N> | NegativeInteger<N>;

function takeInt<const T extends number>(
    int: Integer<T>,
): T extends PositiveInteger<T> ? true : false {
    throw "";
}

function takeNegativeInt<const T extends number>(int: NegativeInteger<T>) {
    takeInt(int);
}

takeInt(-33);

type IntegerObject<T extends Record<string, number>> = {
    [K in keyof T]: Integer<T[K]>;
};
type IntegerArr<T extends number[]> = {
    [K in keyof T]: Integer<T[K]>;
};

takeIntArr([2, 4, 5, -4]);

function takeInts<const T extends Record<string, number>>(
    ints: IntegerObject<T>,
) {}

function takeIntArr<const T extends number[]>(ints: IntegerArr<T>) {}

const h1 = 0xf9;
type x11 = typeof h1;

const v2 = takeInt(9);

const direction = {
    UP: 0,
    DOWN: 1,
    LEFT: 2,
    RIGHT: 3,
} as const;

const variants = ["A", "B"] as const;

// type EnumField<T extends readonly string[]> = {};

type RangedField<A extends number, B extends number> = {
    readonly __ranged_marker: unique symbol;
    min: A;
    max: B;
};

export const INT_MARKER: unique symbol = Symbol("INT");
export const BOOL_MARKER: unique symbol = Symbol("BOOL");
export const ENUM_MARKER: unique symbol = Symbol("ENUM");
export const ARRAY_MARKER: unique symbol = Symbol("ARRAY");
export const OBJECT_MARKER: unique symbol = Symbol("OBJECT");
export type IntField = { readonly __type_marker: typeof INT_MARKER };
export type BoolField = { readonly __type_marker: typeof BOOL_MARKER };
export type EnumField = {
    readonly __type_marker: typeof ENUM_MARKER;
    enums: string[];
};
export type ArrayField<T extends Field> = {
    readonly __type_marker: typeof ARRAY_MARKER;
    item_type: T;
};
export type ObjectField<T extends Record<string, Field>> = {
    readonly __type_marker: typeof OBJECT_MARKER;
    fields: T;
};

export type Field =
    | IntField
    | BoolField
    | EnumField
    | ArrayField<any>
    | ObjectField<any>;

type StepInput = (ObjectField<any> | ObjectField<any>[])[];

export type Path<T extends StepInput, N extends Field> = PathObject<T, N, any>;

type PathObject<T extends StepInput, N extends Field, I extends number> =
    I extends number
    ? T[I] extends ObjectField<any>
        ? { index: I; path: PathArray<T[I], N, any> }
        : T[I] extends N[]
          ? { index: I }
          : "never1"
    : "never0";

type PathArray<
    T extends ObjectField<any>,
    N extends Field,
    P extends string[],
> = P extends [infer H extends string, ...infer R extends string[]]
    ? R[0] extends string
        ? T["fields"][H] extends ObjectField<any>
            ? [H, ...PathArray<T["fields"][H], N, R>]
            : ["never2"]
        : T["fields"][H] extends N
          ? [H]
          : ["never3"]
    : ["never4"];

type test_type = {
    __type_marker: typeof OBJECT_MARKER;
    fields: {
        first: {
            __type_marker: typeof OBJECT_MARKER;
            fields: {
                second: {
                    __type_marker: typeof OBJECT_MARKER;
                    fields: {
                        third: IntField;
                    };
                };
            };
        };
    };
};
let path_test: PathObject<[test_type, test_type[]], IntField, 0> = {
    index: 0,
    path: ["first", "second", "third"],
};

type Any_Test<T extends String> = T extends ("3" | "4") ? [T] : "5";
let any_test: Any_Test<any> = [6]

type SetMapping<T extends StepInput, N extends Field> = {
    d: "SET_MAP";
}; //ToDo
type Count<T extends StepInput> = {
    count: SetMatchingValue<T, Field>;
};
type SetOperatior = "UNION" | "INTERSECTION" | "DIFFERENCE";
type SetOperation<T extends StepInput, N extends Field> = {
    operator: SetOperatior;
    input:
        | SetMatchingValue<T, N>[]
        | SetMatchingValue<
              T,
              { __type_marker: typeof ARRAY_MARKER; item_type: N }
          >;
};
type MathOperator = "PLUS" | "MINUS" | "MULTIPLY" | "DIVIDE_DOWN" | "DIVIDE_UP";
type MathOperation<T extends StepInput> = {
    operator: MathOperator;
    first: NumberMatchingValue<T>;
    second: NumberMatchingValue<T>;
};
type GeneralMatchingOperator = "EQUAL" | "UNEQUAL";
type BoolMatchingOperator = "AND" | "OR";
type NumberMatchingOperator = "EQUALS" | "SMALLER" | "BIGGER";
type SetMatchingOperator = "IS_SUBSET" | "IS_TRUE_SUBSET";
type GeneralMatch<T extends StepInput> = {
    operator: GeneralMatchingOperator;
    first: MatchingValue<T>;
    second: MatchingValue<T>;
};
type BoolMatch<T extends StepInput> = {
    operator: BoolMatchingOperator;
    first: BoolMatchingValue<T>;
    second: BoolMatchingValue<T>;
};
type NumberMatch<T extends StepInput> = {
    operator: NumberMatchingOperator;
    first: NumberMatchingValue<T>;
    second: NumberMatchingValue<T>;
};
type SetMatch<T extends StepInput, N extends Field> = {
    operator: SetMatchingOperator;
    first: SetMatchingValue<T, N>;
    second: SetMatchingValue<T, N>;
};
export type Match<T extends StepInput> =
    | GeneralMatch<T>
    | BoolMatch<T>
    | NumberMatch<T>
    | SetMatch<T, any>;

type BoolMatchingValue<T extends StepInput> = Path<T, BoolField> | Match<T>;
type NumberMatchingValue<T extends StepInput> =
    | Integer<any>
    | Path<T, IntField>
    | MathOperation<T>
    | Count<T>;
type SetMatchingValue<T extends StepInput, N extends Field> =
    | SetOperation<T, N>
    | SetMapping<T, N>
    | Path<T, ArrayField<N>>;
type MatchingValue<T extends StepInput> =
    | BoolMatchingValue<T>
    | NumberMatchingValue<T>
    | Path<T, any>
    | SetMatchingValue<T, Field>;

const x = mapSomething(variants);

function mapSomething<const T extends readonly string[]>(
    direction: T,
): T[number] | undefined {
    return direction[0];
}

export class Rule<const T extends ObjectField<any>> {
    private readonly puzzpt_export = null;

    readonly deduction: Deduction<T>;
    // Rule Placement
    // Renderer

    constructor(data: T) {
        this.deduction = new Deduction(data);
    }
}

export class Deduction<const T extends ObjectField<any>> {
    private readonly puzzpt_export = null;

    readonly data: T;

    constructor(data: T) {
        this.data = data;
    }
}

export class LogicStep<const Input extends ObjectField<any>[], const Output> {
    private readonly puzzpt_export = null;

    readonly match_statement: Match<Input>;
    // output statement

    constructor(match_statement: Match<Input>) {
        this.match_statement = match_statement;
    }
}

export class ScriptModule {
    private readonly puzzpt_export = null;

    readonly author_id: string;
    readonly id: string;
    readonly description: string;

    private constructor(props: {
        author_id: string;
        id: string;
        description: string;
    }) {
        this.author_id = props.author_id;
        this.id = props.id;
        this.description = props.description;
    }

    static create(props: {
        author_id: string;
        id: string;
        description: string;
    }): ScriptModule {
        return new ScriptModule(props);
    }

    createRule<const T extends ObjectField<any>>(props: T): Rule<T> {
        return new Rule<T>(props);
    }

    createDeduction<const T extends ObjectField<any>>(props: T): Deduction<T> {
        return new Deduction<T>(props);
    }

    createLogicStep<const T extends ObjectField<any>[], N>(
        match_statement: Match<T>,
    ): LogicStep<T, N> {
        return new LogicStep(match_statement);
    }
}
