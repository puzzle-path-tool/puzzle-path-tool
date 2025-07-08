// #region [[Definitions]]

const classData = Symbol("classData");

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type TODO = any;
function todo(...args: unknown[]): never {
    throw new Error(`TODO: ${args.join(", ")}`);
}

type RecordType<T = unknown> = Record<string, T>;

function wrapProxyDyn<TBase extends object, TExtra extends RecordType>(
    base: TBase,
    extra: (target: TBase) => TExtra,
): TBase & TExtra {
    return new Proxy(base, {
        get(target, prop, receiver) {
            const extraObj = extra(target);

            if (typeof prop === "string" && prop in extraObj) {
                return extraObj[prop];
            }
            return Reflect.get(target, prop, receiver);
        },
        has(target, p) {
            if (Reflect.has(target, p)) {
                return true;
            }

            const extraObj = extra(target);

            return Reflect.has(extraObj, p);
        },
        ownKeys(target) {
            const extraObj = extra(target);

            return Array.from(
                new Set([
                    ...Reflect.ownKeys(target),
                    ...Reflect.ownKeys(extraObj),
                ]),
            );
        },
        getOwnPropertyDescriptor(target, p) {
            const extraObj = extra(target);

            if (Reflect.has(extraObj, p)) {
                return Reflect.getOwnPropertyDescriptor(extraObj, p);
            }
            return Reflect.getOwnPropertyDescriptor(target, p);
        },
    }) as TBase & TExtra;
}

function wrapProxy<TBase extends object, TExtra extends RecordType>(
    base: TBase,
    extra: TExtra,
): TBase & TExtra {
    return wrapProxyDyn(base, () => extra);
}

function mapFields<TFrom extends RecordType, TTo extends RecordType>(
    obj: TFrom,
    map_key: (
        key: keyof TFrom,
        value: TFrom[keyof TFrom],
    ) => [key: keyof TTo, value: TTo[keyof TTo]][],
): TTo {
    const entries = Object.entries(obj).flatMap(([k, v]) => {
        const key: keyof TFrom = k;
        const value = v as TFrom[keyof TFrom];

        return map_key(key, value);
    });

    return Object.fromEntries(entries) as TTo;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type FieldFunc = (...args: any[]) => FieldType;

function createFieldFunc<F extends FieldFunc>(
    props: { name: string },
    f: F,
): F {
    const wrapper = ((...args: Parameters<F>) => {
        return f(...args); // TODO: Wrap
    }) as F;

    return wrapper;
}

const u2 = createFieldFunc(
    {
        name: "u2",
    },
    (a: number, b: string, c: IntFieldType): IntFieldType => {
        return c;
    },
);

// #endregion
// #region [[Opaque Data test]]

type AData = { name: string; value: number };
class AWrapper {
    private readonly data: AData;
    constructor(data: AData) {
        this.data = data;
    }
    static unwrap(value: AWrapper): AData {
        return value.data;
    }
    static wrap(value: AData): AWrapper {
        return new AWrapper(value);
    }
}
export type A = AWrapper;

export function doStuff(value: A) {
    const v = AWrapper.unwrap(value);

    //TODO
}

// #endregion
// #region [[Opaque Data test 2]]

type BData<T extends RecordType> = { name: string; fields: T };
type BFields<T extends RecordType> = {
    [K in keyof T]: { id: string; value: T[K] };
};

function wrapFields<const T extends RecordType>(
    obj: T,
    id: string,
): BFields<T> {
    return mapFields<T, BFields<T>>(obj, (key, value) => [
        [
            key,
            {
                id: id,
                value: value,
            },
        ],
    ]);
}

class BWrapper<T extends RecordType> {
    private readonly [classData]: BData<T>;
    private constructor(data: BData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends RecordType>(value: B<T>): BData<T> {
        return value[classData];
    }
    static wrap<const T extends RecordType>(value: BData<T>): B<T> {
        return wrapProxy(
            new BWrapper(value),
            wrapFields(value.fields, value.name),
        );
    }
}
export type B<T extends RecordType> = BWrapper<T> & BFields<T>;

const b = BWrapper.wrap({
    name: "SomeName",
    fields: {
        x: 1,
        y: 2,
        value: 3,
        toString: 4,
        _ee2: 5,
        [1.2e2]: 6,
    },
});

console.log({ ...b });
console.log(b.x.id);
console.log(b.x.value);

console.log(b instanceof BWrapper);
console.log(BWrapper.unwrap(b));
// #endregion
// #region [[Field Declarations]]

// #region [Field Type Declaration]

/* eslint-disable @typescript-eslint/no-explicit-any */
type FieldDeclaration =
    | IntFieldDeclaration
    | BoolFieldDeclaration
    | EnumFieldDeclaration<any>
    | ObjectFieldDeclaration<any>
    | ArrayFieldDeclaration<any>;
/* eslint-enable @typescript-eslint/no-explicit-any */

class FieldDeclUtil {
    static isInt(value: unknown): value is IntFieldDeclaration {
        return value instanceof IntFieldDeclarationWrapper;
    }
    static isBool(value: unknown): value is BoolFieldDeclaration {
        return value instanceof BoolFieldDeclarationWrapper;
    }
    static isEnum(
        value: unknown,
    ): value is EnumFieldDeclaration<readonly string[]> {
        return value instanceof EnumFieldDeclarationWrapper;
    }
    static isObject(
        value: unknown,
    ): value is ObjectFieldDeclaration<RecordType<FieldDeclaration>> {
        return value instanceof ObjectFieldDeclarationWrapper;
    }
    static isArray(
        value: unknown,
    ): value is ArrayFieldDeclaration<FieldDeclaration> {
        return value instanceof ArrayFieldDeclarationWrapper;
    }
}

// #endregion
// #region [Int Field Declaration]

type IntFieldDeclarationData = object;
class IntFieldDeclarationWrapper {
    private readonly [classData]: IntFieldDeclarationData;
    private constructor(data: IntFieldDeclarationData) {
        this[classData] = data;
    }
    static unwrap(value: IntFieldDeclaration): IntFieldDeclarationData {
        return value[classData];
    }
    static wrap(value: IntFieldDeclarationData): IntFieldDeclaration {
        return new IntFieldDeclarationWrapper(value);
    }
}
export type IntFieldDeclaration = IntFieldDeclarationWrapper;

// #endregion
// #region [Bool Field Declaration]

type BoolFieldDeclarationData = object;
class BoolFieldDeclarationWrapper {
    private readonly [classData]: BoolFieldDeclarationData;
    private constructor(data: BoolFieldDeclarationData) {
        this[classData] = data;
    }
    static unwrap(value: BoolFieldDeclaration): BoolFieldDeclarationData {
        return value[classData];
    }
    static wrap(value: BoolFieldDeclarationData): BoolFieldDeclaration {
        return new BoolFieldDeclarationWrapper(value);
    }
}
export type BoolFieldDeclaration = BoolFieldDeclarationWrapper;

// #endregion
// #region [Enum Field Declaration]

interface EnumFieldDeclarationData<T extends readonly string[]> {
    values: T;
}
class EnumFieldDeclarationWrapper<T extends readonly string[]> {
    private readonly [classData]: EnumFieldDeclarationData<T>;
    private constructor(data: EnumFieldDeclarationData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends readonly string[]>(
        value: EnumFieldDeclaration<T>,
    ): EnumFieldDeclarationData<T> {
        return value[classData];
    }
    static wrap<const T extends readonly string[]>(
        value: EnumFieldDeclarationData<T>,
    ): EnumFieldDeclaration<T> {
        return new EnumFieldDeclarationWrapper(value);
    }
}

export type EnumFieldDeclaration<T extends readonly string[]> =
    EnumFieldDeclarationWrapper<T>;

// #endregion
// #region [Object Field Declaration]

interface ObjectFieldDeclarationData<T extends RecordType<FieldDeclaration>> {
    fields: T;
}
class ObjectFieldDeclarationWrapper<T extends RecordType<FieldDeclaration>> {
    private readonly [classData]: ObjectFieldDeclarationData<T>;
    private constructor(data: ObjectFieldDeclarationData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends RecordType<FieldDeclaration>>(
        value: ObjectFieldDeclaration<T>,
    ): ObjectFieldDeclarationData<T> {
        return value[classData];
    }
    static wrap<const T extends RecordType<FieldDeclaration>>(
        value: ObjectFieldDeclarationData<T>,
    ): ObjectFieldDeclaration<T> {
        return new ObjectFieldDeclarationWrapper(value);
    }
}
export type ObjectFieldDeclaration<T extends RecordType<FieldDeclaration>> =
    ObjectFieldDeclarationWrapper<T>;

// #endregion
// #region [Array Field Declaration]

interface ArrayFieldDeclarationData<T extends FieldDeclaration> {
    item: T;
}
class ArrayFieldDeclarationWrapper<T extends FieldDeclaration> {
    private readonly [classData]: ArrayFieldDeclarationData<T>;
    private constructor(data: ArrayFieldDeclarationData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldDeclaration>(
        value: ArrayFieldDeclaration<T>,
    ): ArrayFieldDeclarationData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldDeclaration>(
        value: ArrayFieldDeclarationData<T>,
    ): ArrayFieldDeclaration<T> {
        return new ArrayFieldDeclarationWrapper(value);
    }
}
export type ArrayFieldDeclaration<T extends FieldDeclaration> =
    ArrayFieldDeclarationWrapper<T>;

// #endregion

// #endregion
// #region [[Field Types]]

// #region [Field Type]

/* eslint-disable @typescript-eslint/no-explicit-any */
export type FieldType =
    | IntFieldType
    | BoolFieldType
    | EnumFieldType<any>
    | ObjectFieldType<any>
    | ArrayFieldType<any>;
/* eslint-enable @typescript-eslint/no-explicit-any */

class FieldTypeUtil {
    static isInt(value: unknown): value is IntFieldType {
        return value instanceof IntFieldTypeWrapper;
    }
    static isBool(value: unknown): value is BoolFieldType {
        return value instanceof BoolFieldTypeWrapper;
    }
    static isEnum(value: unknown): value is EnumFieldType<readonly string[]> {
        return value instanceof EnumFieldTypeWrapper;
    }
    static isObject(
        value: unknown,
    ): value is ObjectFieldType<RecordType<FieldType>> {
        return value instanceof ObjectFieldTypeWrapper;
    }
    static isArray(value: unknown): value is ArrayFieldType<FieldType> {
        return value instanceof ArrayFieldTypeWrapper;
    }
}

// #endregion
// #region [Int Field Declaration]

type IntFieldTypeData = object;

class IntFieldTypeWrapper {
    private readonly [classData]: IntFieldTypeData;
    private constructor(data: IntFieldTypeData) {
        this[classData] = data;
    }
    static unwrap(value: IntFieldType): IntFieldTypeData {
        return value[classData];
    }
    static wrap(value: IntFieldTypeData): IntFieldType {
        return new IntFieldTypeWrapper(value);
    }
}
export type IntFieldType = IntFieldTypeWrapper;

// #endregion
// #region [Bool Field Declaration]

type BoolFieldTypeData = object;
class BoolFieldTypeWrapper {
    private readonly [classData]: BoolFieldTypeData;
    private constructor(data: BoolFieldTypeData) {
        this[classData] = data;
    }
    static unwrap(value: BoolFieldType): BoolFieldTypeData {
        return value[classData];
    }
    static wrap(value: BoolFieldTypeData): BoolFieldType {
        return new BoolFieldTypeWrapper(value);
    }
}
export type BoolFieldType = BoolFieldTypeWrapper;

// #endregion
// #region [Enum Field Declaration]

interface EnumFieldTypeData<T extends readonly string[]> {
    values: T;
}
class EnumFieldTypeWrapper<T extends readonly string[]> {
    private readonly [classData]: EnumFieldTypeData<T>;
    private constructor(data: EnumFieldTypeData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends readonly string[]>(
        value: EnumFieldType<T>,
    ): EnumFieldTypeData<T> {
        return value[classData];
    }
    static wrap<const T extends readonly string[]>(
        value: EnumFieldTypeData<T>,
    ): EnumFieldType<T> {
        return new EnumFieldTypeWrapper(value);
    }
}

export type EnumFieldType<T extends readonly string[]> =
    EnumFieldTypeWrapper<T>;

// #endregion
// #region [Object Field Declaration]

interface ObjectFieldTypeData<T extends RecordType<FieldType>> {
    fields: T;
}
class ObjectFieldTypeWrapper<T extends RecordType<FieldType>> {
    private readonly [classData]: ObjectFieldTypeData<T>;
    private constructor(data: ObjectFieldTypeData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends RecordType<FieldType>>(
        value: ObjectFieldType<T>,
    ): ObjectFieldTypeData<T> {
        return value[classData];
    }
    static wrap<const T extends RecordType<FieldType>>(
        value: ObjectFieldTypeData<T>,
    ): ObjectFieldType<T> {
        return new ObjectFieldTypeWrapper(value);
    }
}
export type ObjectFieldType<T extends RecordType<FieldType>> =
    ObjectFieldTypeWrapper<T>;

// #endregion
// #region [Array Field Declaration]

interface ArrayFieldTypeData<T extends FieldType> {
    item: T;
}
class ArrayFieldTypeWrapper<T extends FieldType> {
    private readonly [classData]: ArrayFieldTypeData<T>;
    private constructor(data: ArrayFieldTypeData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(
        value: ArrayFieldType<T>,
    ): ArrayFieldTypeData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(
        value: ArrayFieldTypeData<T>,
    ): ArrayFieldType<T> {
        return new ArrayFieldTypeWrapper(value);
    }
}
export type ArrayFieldType<T extends FieldType> = ArrayFieldTypeWrapper<T>;

// #endregion

// #endregion
// #region [[Var Value]]

// #region [Variable]
interface RefVariableData<T extends FieldType> {
    values: T;
}
class RefVariableWrapper<T extends FieldType> {
    private readonly [classData]: RefVariableData<T>;
    private constructor(data: RefVariableData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(
        value: RefVariable<T>,
    ): RefVariableData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(
        value: RefVariableData<T>,
    ): RefVariable<T> {
        return new RefVariableWrapper(value);
    }
}

type RefVariable<T extends FieldType> = RefVariableWrapper<T>;

type PartialRefVariable<T> = T extends IntFieldType
    ? number
    : T extends BoolFieldType
      ? boolean
      : T extends ArrayFieldType<infer TItem extends FieldType>
        ? Var<TItem>[]
        : T extends ObjectFieldType<infer TObj extends RecordType<FieldType>>
          ? {
                [K in keyof TObj]: Var<TObj[K]>;
            }
          : never;

interface TypeHolder<T> {
    type: T;
}

export type Var<T> = T extends FieldType
    ? RefVariable<T> | PartialRefVariable<T>
    : T extends TypeHolder<infer TInner extends FieldType>
      ? Var<TInner>
      : never;

const x = ObjectFieldTypeWrapper.wrap({
    fields: {
        a1: IntFieldTypeWrapper.wrap({}),
        a2: IntFieldTypeWrapper.wrap({}),
    },
});

function f11(p: Var<typeof x>, o: "==", p2: Var<typeof x>) {
    console.log(p);
}

// f11(VariableWrapper.wrap({ values: x }));
f11(
    {
        a1: 1,
        a2: RefVariableWrapper.wrap({ values: IntFieldTypeWrapper.wrap({}) }),
    },
    "==",
    RefVariableWrapper.wrap({ values: x }),
);
f11(
    {
        a1: 1,
        a2: 2,
    },
    "==",
    {
        a1: 1,
        a2: 23,
    },
);

// const x12 = {x: (a: number, b: number): number => todo()} as const;

// function f12(p: typeof makeInfix(x12)) {
//     todo();
// }

// #endregion
// #region [Int Op]

type IntCmpOp = "==" | "!=" | ">=" | "<=" | ">" | "<" | "**";
type IntMathOp = "+" | "-" | "*" | "//" | "mod" | "rem" | "**";
type IntFoldOp = "+" | "*" | "min" | "max";
type IntAllOp = "==" | "!=";

const intOp = {
    cmp: (
        a: Var<IntFieldType>,
        o: IntCmpOp,
        b: Var<IntFieldType>,
    ): Var<BoolFieldType> => {
        todo();
    },

    math: (
        a: Var<IntFieldType>,
        o: IntMathOp,
        b: Var<IntFieldType>,
    ): Var<IntFieldType> => {
        todo();
    },

    fold: (
        o: IntFoldOp,
        a: Var<ArrayFieldType<IntFieldType>>,
    ): Var<IntFieldType> => {
        todo();
    },

    sum: (...items: Var<IntFieldType>[]): Var<IntFieldType> => {
        todo();
    },

    product: (...items: Var<IntFieldType>[]): Var<IntFieldType> => {
        todo();
    },

    min: (...items: Var<IntFieldType>[]): Var<IntFieldType> => {
        todo();
    },

    max: (...items: Var<IntFieldType>[]): Var<IntFieldType> => {
        todo();
    },

    all: (
        o: IntAllOp,
        items: Var<ArrayFieldType<IntFieldType>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    equal: (...items: Var<IntFieldType>[]): Var<BoolFieldType> => {
        todo();
    },

    none_equal: (...items: Var<IntFieldType>[]): Var<BoolFieldType> => {
        todo();
    },
};

// #endregion
// #region [Set Op]

type SetCmpOp =
    | "=="
    | "!="
    | "subset of"
    | "superset of"
    | "true subset of"
    | "true superset of"
    | "disjoint with";

type SetJoinOp =
    | "union"
    | "intersect"
    | "without"
    | "subtracted from"
    | "disjunctive union";

type SetFoldOp = "==" | "!=" | "union" | "intersect" | "disjunctive union";

type SetAllOp = "==" | "!=" | "disjoint";

const setOp = {
    cmp: <T extends FieldType>(
        a: Var<ArrayFieldType<T>>,
        o: SetCmpOp,
        b: Var<ArrayFieldType<T>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    element_of: <T extends FieldType>(
        a: Var<T>,
        o: "element of",
        b: Var<ArrayFieldType<T>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    contains: <T extends FieldType>(
        a: Var<ArrayFieldType<T>>,
        o: "contains",
        b: Var<T>,
    ): Var<BoolFieldType> => {
        todo();
    },

    join: <T extends FieldType>(
        a: Var<ArrayFieldType<T>>,
        o: SetJoinOp,
        b: Var<ArrayFieldType<T>>,
    ): Var<ArrayFieldType<T>> => {
        todo();
    },

    fold: <T extends FieldType>(
        o: SetFoldOp,
        items: Var<ArrayFieldType<ArrayFieldType<T>>>,
    ): Var<ArrayFieldType<T>> => {
        todo();
    },

    union: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<ArrayFieldType<T>> => {
        todo();
    },

    intersect: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<ArrayFieldType<T>> => {
        todo();
    },

    disjunctive_union: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<ArrayFieldType<T>> => {
        todo();
    },

    all: <T extends FieldType>(
        o: SetAllOp,
        items: Var<ArrayFieldType<ArrayFieldType<T>>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    equal: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<BoolFieldType> => {
        todo();
    },

    none_equal: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<BoolFieldType> => {
        todo();
    },

    all_disjoint: <T extends FieldType>(
        ...items: Var<ArrayFieldType<T>>[]
    ): Var<BoolFieldType> => {
        todo();
    },

    size: <T extends FieldType>(
        item: Var<ArrayFieldType<T>>,
    ): Var<IntFieldType> => {
        todo();
    },

    map: <T extends FieldType, R extends FieldType>(
        item: Var<ArrayFieldType<T>>,
        f: (value: Var<T>) => Var<R>,
    ): Var<ArrayFieldType<R>> => {
        todo();
    },
};

// #endregion
// #region [Bool Op]

type BoolCmpOp = "==" | "!=";
type BoolLogicOp = "or" | "and" | "xor" | "nor" | "nand" | "xnor";
type BoolSetOp = "all" | "any" | "none" | BoolCmpOp | BoolLogicOp;
type BoolAllOp = "==" | "!=" | "true" | "false";

const boolOp = {
    cmp: (
        a: Var<BoolFieldType>,
        o: BoolCmpOp,
        b: Var<BoolFieldType>,
    ): Var<BoolFieldType> => {
        todo();
    },

    logic: (
        a: Var<BoolFieldType>,
        o: BoolLogicOp,
        b: Var<BoolFieldType>,
    ): Var<BoolFieldType> => {
        todo();
    },

    fold: (
        o: BoolSetOp,
        a: Var<ArrayFieldType<BoolFieldType>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    all: (
        o: BoolAllOp,
        items: Var<ArrayFieldType<BoolFieldType>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    equal: (...items: Var<BoolFieldType>[]): Var<BoolFieldType> => {
        todo();
    },

    none_equal: (...items: Var<BoolFieldType>[]): Var<BoolFieldType> => {
        todo();
    },

    none: (...items: Var<BoolFieldType>[]): Var<BoolFieldType> => {
        todo();
    },

    and: (...items: Var<BoolFieldType>[]): Var<BoolFieldType> => {
        todo();
    },

    or: (...items: Var<BoolFieldType>[]): Var<BoolFieldType> => {
        todo();
    },
};

// #endregion
// #region [Obj Op]

type ObjCmpOp = "==" | "!=";
type ObjAllOp = "==" | "!=";

const objOp = {
    cmp: <T extends RecordType<FieldType>>(
        a: Var<ObjectFieldType<T>>,
        o: ObjCmpOp,
        b: Var<ObjectFieldType<T>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    all: <T extends RecordType<FieldType>>(
        o: ObjAllOp,
        items: Var<ArrayFieldType<ObjectFieldType<T>>>,
    ): Var<BoolFieldType> => {
        todo();
    },

    equal: <T extends RecordType<FieldType>>(
        ...items: Var<ObjectFieldType<T>>[]
    ): Var<BoolFieldType> => {
        todo();
    },

    none_equal: <T extends RecordType<FieldType>>(
        ...items: Var<ObjectFieldType<T>>[]
    ): Var<BoolFieldType> => {
        todo();
    },
};

// #endregion
// #region [Quantor Op]

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Matcher = any;

const quantorOp = {
    all: (f: (matcher: Matcher) => void): Var<BoolFieldType> => {
        todo();
    },
    exists: (f: (matcher: Matcher) => void): Var<BoolFieldType> => {
        todo();
    },
};

// #endregion
// #region [Op]

// #endregion

// #endregion
// #region [[Test]]

const field = {
    decl: {
        int: IntFieldDeclarationWrapper,
        bool: BoolFieldDeclarationWrapper,
        enum: EnumFieldDeclarationWrapper,
        obj: ObjectFieldDeclarationWrapper,
        arr: ArrayFieldDeclarationWrapper,
    },
} as const;

const b1 = field.decl.obj.wrap({
    fields: {
        a: field.decl.int.wrap({}),
        b: field.decl.obj.wrap({
            fields: {
                c: field.decl.int.wrap({}),
                d: field.decl.int.wrap({}),
                e: field.decl.obj.wrap({
                    fields: {
                        f: field.decl.int.wrap({}),
                        g: field.decl.int.wrap({}),
                    },
                }),
            },
        }),
        aa: field.decl.arr.wrap({
            item: field.decl.int.wrap({}),
        }),
        ab: field.decl.arr.wrap({
            item: field.decl.obj.wrap({
                fields: {
                    a: field.decl.arr.wrap({
                        item: field.decl.arr.wrap({
                            item: field.decl.int.wrap({}),
                        }),
                    }),
                },
            }),
        }),
        ac: field.decl.obj.wrap({
            fields: {
                a: field.decl.enum.wrap({
                    values: ["1", "2", "3"],
                }),
            },
        }),
    },
});

const b2 = field.decl.obj.wrap({
    fields: {
        a: field.decl.enum.wrap({
            values: ["1", "2", "3"],
        }),
        b: field.decl.bool.wrap({}),
    },
});

// const f = makeInfix({
//     x1: (a: string, b: string): string => {
//         return a + b;
//     },
//     x2: (a: number, b: string): number => {
//         return a;
//     },
//     x3: (a: number, b: number): number => {
//         return a - b;
//     },
// });

// const f2 = makePrefix({
//     x1: (x: string): string => {
//         return x + ": x1";
//     },
//     x2: (x: number): number => {
//         return x + 3;
//     },
//     x3: (x: number): number => {
//         return x - 5;
//     },
//     x4: (x: number): string => {
//         return `a4: ${x}`;
//     },
// });

// console.log(f("Hello", "x1", "World"));
// console.log(f(1, "x2", "Ignore"));
// console.log(f(10, "x3", 2));

// console.log(f2("x1", "Hello"));
// console.log(f2("x2", 3));
// console.log(f2("x3", 10));
// console.log(f2("x4", 100));

const int = {
    field: () => todo(),
    type: todo(), //
    const: () => todo(), //
    op: {},
};

type TypeDef<Props, F, T, VarT, Op> = {
    field: (props?: Props) => F;
    type: T;
    const: (value: VarT) => VarT;
    op: Op;
};

// type VarOf<T extends TypeDef>

// const int2 = makeType({
//     fields: {},
//     op: {
//         do: makeInfix({
//             "+": (a: number, b: number): number => {
//                 return a + b;
//             },
//         }),
//     },
// });

const name12 = {
    x: 1,
    type: 3,
};

// function name12point5(item: Var<typeof name12.type>) {
//     todo();
// }
// function name12point6(item: VarOf<typeof name12>) {
//     todo();
// }

const name13: typeof name12.x = 3;

// #endregion
