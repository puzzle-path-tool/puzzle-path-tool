// #region [[Definitions]]

const classData = Symbol("classData");

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
type InfixFuncDefinition = Record<string, (a: any, b: any) => any>;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type InfixFuncArgA<F> = F extends (a: infer A, b: any) => any ? A : never;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type InfixFuncArgB<F> = F extends (a: any, b: infer B) => any ? B : never;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type InfixFuncReturn<F> = F extends (a: any, b: any) => infer R ? R : never;

interface InfixFunc<F extends InfixFuncDefinition> {
    <O extends keyof F>(
        a: NoInfer<InfixFuncArgA<F[O]>>,
        o: O,
        b: NoInfer<InfixFuncArgB<F[O]>>,
    ): NoInfer<InfixFuncReturn<F[O]>>;
}

function makeInfix<const F extends InfixFuncDefinition>(
    funcs: F,
): InfixFunc<F> {
    return (a, o, b) => {
        return funcs[o]?.(a, b);
    };
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type PrefixFuncDefinition = Record<string, (x: any) => any>;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type PrefixFuncArg<F> = F extends (x: infer X) => any ? X : never;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type PrefixFuncReturn<F> = F extends (x: any) => infer R ? R : never;

interface PrefixFunc<F extends PrefixFuncDefinition> {
    <O extends keyof F>(
        o: O,
        x: NoInfer<PrefixFuncArg<F[O]>>,
    ): NoInfer<PrefixFuncReturn<F[O]>>;
}

function makePrefix<const F extends PrefixFuncDefinition>(
    funcs: F,
): PrefixFunc<F> {
    return (o, x) => {
        return funcs[o]?.(x);
    };
}

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
type FieldType =
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
interface VariableData<T extends FieldType> {
    values: T;
}
class VariableWrapper<T extends FieldType> {
    private readonly [classData]: VariableData<T>;
    private constructor(data: VariableData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(value: Variable<T>): VariableData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(
        value: VariableData<T>,
    ): Variable<T> {
        return new VariableWrapper(value);
    }
}

export type Variable<T extends FieldType> = VariableWrapper<T>;

type ConstOf<T> = T extends IntFieldType
    ? number
    : T extends BoolFieldType
      ? boolean
      : T extends ArrayFieldType<infer TItem extends FieldType>
        ? ConstOf<TItem>[]
        : T extends ObjectFieldType<infer TObj extends RecordType<FieldType>>
          ? {
                [K in keyof TObj]: ConstOf<TObj[K]>;
            }
          : never;

type IntoVar<T extends FieldType> = Variable<T> | ConstOf<T>;

const x = ObjectFieldTypeWrapper.wrap({
    fields: {
        a1: IntFieldTypeWrapper.wrap({}),
    },
});

function f11(p: IntoVar<typeof x>) {
    todo();
}

f11(VariableWrapper.wrap({ values: x }));
f11({
    a1: 1,
});

// #endregion
// #region [Cmp Op]

class CmpOp {
    private constructor() {}

    static readonly do = makeInfix({
        "==": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "!=": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        ">=": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "<=": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        ">": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "<": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
    });
}

// #endregion
// #region [Int Op]

class IntOp {
    private constructor() {}

    static readonly do = makeInfix({
        "+": (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        "-": (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        "*": (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        "//": (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        mod: (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        rem: (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
        "**": (
            a: Variable<IntFieldType>,
            b: Variable<IntFieldType>,
        ): Variable<IntFieldType> => {
            todo();
        },
    });
    static readonly fold = makePrefix({
        "+": (
            items: Variable<ArrayFieldType<IntFieldType>>,
        ): Variable<IntFieldType> => {
            todo();
        },
        "*": (
            items: Variable<ArrayFieldType<IntFieldType>>,
        ): Variable<IntFieldType> => {
            todo();
        },
    });
}

// #endregion
// #region [Set Op]

class SetOp {
    private constructor() {}

    static readonly do = makeInfix({
        "subset of": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "superset of": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "true subset of": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "true superset of": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "disjoint with": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        "element of": <T extends FieldType>(
            a: Variable<T>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        contains: <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<T>,
        ): Variable<BoolFieldType> => {
            todo();
        },
        union: <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        intersect: <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        without: <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        "subtracted from": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        "disjunctive union": <T extends FieldType>(
            a: Variable<ArrayFieldType<T>>,
            b: Variable<ArrayFieldType<T>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
    });

    static readonly fold = makePrefix({
        union: <T extends FieldType>(
            items: Variable<ArrayFieldType<ArrayFieldType<T>>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        intersect: <T extends FieldType>(
            items: Variable<ArrayFieldType<ArrayFieldType<T>>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
        "disjunctive union": <T extends FieldType>(
            items: Variable<ArrayFieldType<ArrayFieldType<T>>>,
        ): Variable<ArrayFieldType<T>> => {
            todo();
        },
    });

    static union<T extends FieldType>(
        ...items: Variable<ArrayFieldType<T>>[]
    ): Variable<ArrayFieldType<T>> {
        todo();
    }

    static intersect<T extends FieldType>(
        ...items: Variable<ArrayFieldType<T>>[]
    ): Variable<ArrayFieldType<T>> {
        todo();
    }

    static disjunctive_union<T extends FieldType>(
        ...items: Variable<ArrayFieldType<T>>[]
    ): Variable<ArrayFieldType<T>> {
        todo();
    }
}

// #endregion
// #region [Op]

class Op {
    private constructor() {}
    static readonly cmp = CmpOp;
    static readonly int = IntOp;
    static readonly set = SetOp;
}

export const op = Op;

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

const f = makeInfix({
    x1: (a: string, b: string): string => {
        return a + b;
    },
    x2: (a: number, b: string): number => {
        return a;
    },
    x3: (a: number, b: number): number => {
        return a - b;
    },
});

const f2 = makePrefix({
    x1: (x: string): string => {
        return x + ": x1";
    },
    x2: (x: number): number => {
        return x + 3;
    },
    x3: (x: number): number => {
        return x - 5;
    },
    x4: (x: number): string => {
        return `a4: ${x}`;
    },
});

console.log(f("Hello", "x1", "World"));
console.log(f(1, "x2", "Ignore"));
console.log(f(10, "x3", 2));

console.log(f2("x1", "Hello"));
console.log(f2("x2", 3));
console.log(f2("x3", 10));
console.log(f2("x4", 100));

const int = {
    field: () => todo(),
    var: () => todo(),
    const: () => todo(),
    op: {},
};

// #endregion
