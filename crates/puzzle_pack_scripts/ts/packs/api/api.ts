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
export type ArrayField = {
    readonly __type_marker: typeof ARRAY_MARKER;
    item_type: Field;
};
export type ObjectField = {
    readonly __type_marker: typeof OBJECT_MARKER;
    fields: Record<string, Field>;
};

export type Field = IntField | BoolField | EnumField | ArrayField | ObjectField;

export type NumberPath<
    T extends (ObjectField | ObjectField[])[],
    I extends Integer<number>,
    P extends string[],
> = T[I] extends ObjectField 
? { index: I; path: P, d: "NUMBER_PATH" } 
: never;
export type BoolPath<
    T extends (ObjectField | ObjectField[])[],
    I extends Integer<number>,
    P extends string[],
> = T[I] extends ObjectField ? { index: I; path: P, d: "BOOL_PATH" } : never;
export type SetPath<
    T extends (ObjectField | ObjectField[])[],
    N extends MatchingValue<T>,
    I extends Integer<number>,
    P extends string[],
> = T[I] extends ObjectField ? { index: I; path: P, d: "SET_PATH" } : never;
export type Path<
    T extends (ObjectField | ObjectField[])[],
    I extends Integer<number>,
    P extends string[],
> = T[I] extends ObjectField ? { index: I; path: P } : never;

export type SetMapping<
    T extends (ObjectField | ObjectField[])[],
    N extends MatchingValue<T>,
> = {
    d: "SET_MAP";
}; //ToDo
export type Count<T extends (ObjectField | ObjectField[])[]> = {
    count: SetMatchingValue<T, MatchingValue<T>>;
};
export type SetOperatior = "UNION" | "INTERSECTION" | "DIFFERENCE";
export type SetOperation<
    T extends (ObjectField | ObjectField[])[],
    N extends MatchingValue<T>,
> = {
    operator: SetOperatior;
    input:
        | SetMatchingValue<T, N>[]
        | SetMatchingValue<T, SetMatchingValue<T, N>>;
};
export type MathOperator =
    | "PLUS"
    | "MINUS"
    | "MULTIPLY"
    | "DIVIDE_DOWN"
    | "DIVIDE_UP";
export type MathOperation<T extends (ObjectField | ObjectField[])[]> = {
    operator: MathOperator;
    first: NumberMatchingValue<T>;
    second: NumberMatchingValue<T>;
};
export type SetInput<
    T extends (ObjectField | ObjectField[])[],
    N extends MatchingValue<T>,
> = {
    first: SetMatchingValue<T, N>;
    second: SetMatchingValue<T, N>;
};

export type GeneralMatchingOperator = "EQUAL" | "UNEQUAL";
export type BoolMatchingOperator = "AND" | "OR";
export type NumberMatchingOperator = "EQUALS" | "SMALLER" | "BIGGER";
export type SetMatchingOperator = "IS_SUBSET" | "IS_TRUE_SUBSET";
export type GeneralMatch<T extends (ObjectField | ObjectField[])[]> = {
    operator: GeneralMatchingOperator;
    first: MatchingValue<T>;
    second: MatchingValue<T>;
};
export type BoolMatch<T extends (ObjectField | ObjectField[])[]> = {
    operator: BoolMatchingOperator;
    first: BoolMatchingValue<T>;
    second: BoolMatchingValue<T>;
};
export type NumberMatch<T extends (ObjectField | ObjectField[])[]> = {
    operator: NumberMatchingOperator;
    first: NumberMatchingValue<T>;
    second: NumberMatchingValue<T>;
};
export type SetMatch<T extends (ObjectField | ObjectField[])[]> = {
    operator: SetMatchingOperator;
    input: SetInput<T, MatchingValue<T>>;
};
export type Match<T extends (ObjectField | ObjectField[])[]> =
    | GeneralMatch<T>
    | BoolMatch<T>
    | NumberMatch<T>
    | SetMatch<T>;

type BoolMatchingValue<T extends (ObjectField | ObjectField[])[]> =
    | BoolPath<T, Integer<number>, string[]>
    | Match<T>;
type NumberMatchingValue<T extends (ObjectField | ObjectField[])[]> =
    | NumberPath<T, Integer<number>, string[]>
    | MathOperation<T>
    | Integer<number>
    | Count<T>;
type SetMatchingValue<
    T extends (ObjectField | ObjectField[])[],
    N extends MatchingValue<T>,
> = SetOperation<T, N> | SetPath<T, N, Integer<number>, string[]> | SetMapping<T, N>;
type MatchingValue<T extends (ObjectField | ObjectField[])[]> =
    | BoolMatchingValue<T>
    | NumberMatchingValue<T>
    | Path<T, Integer<number>, string[]>
    | SetMatchingValue<
          T,
          BoolMatchingValue<T> | NumberMatchingValue<T> | Path<T, Integer<number>, string[]>
      >;

const x = mapSomething(variants);

function mapSomething<const T extends readonly string[]>(
    direction: T,
): T[number] | undefined {
    return direction[0];
}

export class Rule<const T extends ObjectField> {
    private readonly puzzpt_export = null;

    readonly deduction: Deduction<T>;
    // Rule Placement
    // Renderer

    constructor(data: T) {
        this.deduction = new Deduction(data);
    }
}

export class Deduction<const T extends ObjectField> {
    private readonly puzzpt_export = null;

    readonly data: T;

    constructor(data: T) {
        this.data = data;
    }
}

export class LogicStep<const Input extends ObjectField[], const Output> {
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

    createRule<const T extends ObjectField>(props: T): Rule<T> {
        return new Rule<T>(props);
    }

    createDeduction<const T extends ObjectField>(props: T): Deduction<T> {
        return new Deduction<T>(props);
    }

    createLogicStep<const T extends ObjectField[], N>(
        match_statement: Match<T>,
    ): LogicStep<T, N> {
        return new LogicStep(match_statement);
    }
}
