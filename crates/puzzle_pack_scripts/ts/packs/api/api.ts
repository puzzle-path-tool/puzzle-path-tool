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

const INT_MARKER: unique symbol = Symbol("INT");
const BOOL_MARKER: unique symbol = Symbol("BOOL");
const ENUM_MARKER: unique symbol = Symbol("ENUM");
const ARRAY_MARKER: unique symbol = Symbol("ARRAY");
const OBJECT_MARKER: unique symbol = Symbol("OBJECT");
type IntField = { readonly __type_marker: typeof INT_MARKER };
type BoolField = { readonly __type_marker: typeof BOOL_MARKER };
type EnumField = {
    readonly __type_marker: typeof ENUM_MARKER;
    enums: string[];
};
type ArrayField = {
    readonly __type_marker: typeof ARRAY_MARKER;
    item_type: Field;
};
type ObjectField = {
    readonly __type_marker: typeof OBJECT_MARKER;
    fields: Record<string, Field>;
};

type Field = IntField | BoolField | EnumField | ArrayField | ObjectField;

type NumberPath<T extends ObjectField[]> = {}; //ToDo
type BoolPath<T extends ObjectField[]> = {}; //ToDo
type Path<T extends ObjectField[]> = {} | NumberPath<T> | BoolPath<T>; //ToDo

type SetBuilder<T extends ObjectField[]> = {}; //ToDo

type MathOperator = "PLUS" | "MINUS" | "MULTIPLY" | "DIVIDE_DOWN" | "DIVIDE_UP";
type MathOperation<T extends ObjectField[]> = {
    operator: MathOperator;
    input: NumberInput<T>;
};
type NumberInput<T extends ObjectField[]> = {
    first: NumberPath<T> | MathOperation<T> | Integer<number>;
    second: NumberPath<T> | MathOperation<T> | Integer<number>;
};
type BoolInput<T extends ObjectField[]> = {
    first: BoolPath<T> | Match<T>;
    second: BoolPath<T> | Match<T>;
};
type SetInput<T extends ObjectField[]> = {
    first: SetBuilder<T>;
    second: SetBuilder<T>;
};
type ObjectInput<T extends ObjectField[]> = { first: Path<T>; second: Path<T> };

type GeneralMatchingOperator = "EQUAL" | "UNEQUAL";
type BoolMatchingOperator = "AND" | "OR";
type NumberMatchingOperator = "EQUALS" | "SMALLER" | "BIGGER";
type SetMatchingOperator = "IS_SUBSET" | "IS_TRUE_SUBSET";
type GeneralMatch<T extends ObjectField[]> = {
    operator: GeneralMatchingOperator;
    input: NumberInput<T> | ObjectInput<T> | BoolInput<T>;
};
type BoolMatch<T extends ObjectField[]> = {
    operator: BoolMatchingOperator;
    input: BoolInput<T>;
};
type NumberMatch<T extends ObjectField[]> = {
    operator: NumberMatchingOperator;
    input: NumberInput<T>;
};
type SetMatch<T extends ObjectField[]> = {
    operator: SetMatchingOperator;
    input: SetInput<T>;
};
type Match<T extends ObjectField[]> =
    | GeneralMatch<T>
    | BoolMatch<T>
    | NumberMatch<T>
    | SetMatch<T>;

const x = mapSomething(variants);

function mapSomething<const T extends readonly string[]>(
    direction: T,
): T[number] | undefined {
    return direction[0];
}

export class Rule<const T> {
    private readonly puzzpt_export = null;

    readonly deduction: Deduction<T>;
    // Rule Placement
    // Renderer

    constructor(data: T) {
        this.deduction = new Deduction(data);
    }
}

export class Deduction<const T> {
    private readonly puzzpt_export = null;

    readonly data: T;

    constructor(data: T) {
        this.data = data;
    }
}

export class LogicStep {
    private readonly puzzpt_export = null;

    // match statement
    // output statement
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

    createRule<const T>(props: T): Rule<T> {
        return new Rule<T>(props);
    }

    createDeduction<const T>(props: T): Deduction<T> {
        return new Deduction<T>(props);
    }

    createLogicStep(): LogicStep {
        return new LogicStep();
    }
}
