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

type IntObj = {};

const numbers = [22] as const;

takeInts({
    x: 4,
    y: 1,
});

takeIntArr([2, 4, 5, -4]);

function takeInts<const T extends Record<string, number>>(
    ints: IntegerObject<T>,
) {}

function takeIntArr<const T extends number[]>(ints: IntegerArr<T>) {}

const h1 = 0xf9;
type x = typeof h1;

const v2 = takeInt(9);

const direction = {
    UP: 0,
    DOWN: 1,
    LEFT: 2,
    RIGHT: 2,
} as const;

const x = mapSomething(["A", "B"]);

function mapSomething<const T extends readonly string[]>(
    direction: T,
): T[number] | undefined {
    return direction[0];
}

// type Int = { readonly __marker_rule: "Int" };
// type Position = { readonly __marker_rule: "Position" };
// type FieldArray<T extends Field<?>> = { readonly __marker_rule: "FieldArray", item: T };
// type FieldObject<O extends {[key: string]: Field<?>}> = { readonly __marker_rule: "FieldObject", fields: O };

// export class Field<T> {
//     static int(): Field<Int> {
//         return new Field();
//     }
//     static position(): Field<Position> {
//         return new Field();
//     }
//     static array<const T extends Field<?>>(field: T): Field<FieldArray<T>> {
//         return new Field();
//     }
// }

export class Rule {
    private readonly puzzpt_export = null;

    // Deduction
    // Renderer
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

    createRule(): Rule {
        return new Rule();
    }

    // createDeduction<const T>(props: T): Deduction {
    //     return new Deduction();
    // }

    createLogicStep(): LogicStep {
        return new LogicStep();
    }
}
