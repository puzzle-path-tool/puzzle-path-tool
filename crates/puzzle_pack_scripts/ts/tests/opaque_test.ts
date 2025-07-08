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
    (a: number, b: string, c: Int): Int => {
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
// #region [[Field Types]]

// #region [Field Type]

/* eslint-disable @typescript-eslint/no-explicit-any */
export type FieldType = Int | Bool | Enum<any> | Object<any> | Set<any>;
/* eslint-enable @typescript-eslint/no-explicit-any */

class FieldTypeUtil {
    static isInt(value: unknown): value is Int {
        return value instanceof IntClass;
    }
    static isBool(value: unknown): value is Bool {
        return value instanceof BoolClass;
    }
    static isEnum(value: unknown): value is Enum<readonly string[]> {
        return value instanceof EnumClass;
    }
    static isObject(value: unknown): value is Object<RecordType<FieldType>> {
        return value instanceof ObjectClass;
    }
    static isArray(value: unknown): value is Set<FieldType> {
        return value instanceof SetClass;
    }
}

// #endregion
// #region [Int Field Declaration]

type IntClassData = object;

class IntClass {
    private readonly [classData]: IntClassData;
    private constructor(data: IntClassData) {
        this[classData] = data;
    }
    static unwrap(value: Int): IntClassData {
        return value[classData];
    }
    static wrap(value: IntClassData): Int {
        return new IntClass(value);
    }
}
export type Int = IntClass;

// #endregion
// #region [Bool Field Declaration]

type BoolClassData = object;
class BoolClass {
    private readonly [classData]: BoolClassData;
    private constructor(data: BoolClassData) {
        this[classData] = data;
    }
    static unwrap(value: Bool): BoolClassData {
        return value[classData];
    }
    static wrap(value: BoolClassData): Bool {
        return new BoolClass(value);
    }
}
export type Bool = BoolClass;

// #endregion
// #region [Enum Field Declaration]

interface EnumClassData<T extends readonly string[]> {
    values: T;
}
class EnumClass<T extends readonly string[]> {
    private readonly [classData]: EnumClassData<T>;
    private constructor(data: EnumClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends readonly string[]>(
        value: Enum<T>,
    ): EnumClassData<T> {
        return value[classData];
    }
    static wrap<const T extends readonly string[]>(
        value: EnumClassData<T>,
    ): Enum<T> {
        return new EnumClass(value);
    }
}

export type Enum<T extends readonly string[]> = EnumClass<T>;

// #endregion
// #region [Object Field Declaration]

interface ObjectClassData<T extends RecordType<FieldType>> {
    fields: T;
}
class ObjectClass<T extends RecordType<FieldType>> {
    private readonly [classData]: ObjectClassData<T>;
    private constructor(data: ObjectClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends RecordType<FieldType>>(
        value: Object<T>,
    ): ObjectClassData<T> {
        return value[classData];
    }
    static wrap<const T extends RecordType<FieldType>>(
        value: ObjectClassData<T>,
    ): Object<T> {
        return new ObjectClass(value);
    }
}
export type Object<T extends RecordType<FieldType>> = ObjectClass<T>;

// #endregion
// #region [Array Field Declaration]

interface ArrayClassData<T extends FieldType> {
    item: T;
}
class SetClass<T extends FieldType> {
    private readonly [classData]: ArrayClassData<T>;
    private constructor(data: ArrayClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(value: Set<T>): ArrayClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(value: ArrayClassData<T>): Set<T> {
        return new SetClass(value);
    }
}
export type Set<T extends FieldType> = SetClass<T>;

// #endregion

// #endregion
// #region [[Var Value]]

// #region [Variable]
interface VarClassData<T extends FieldType> {
    values: T;
}
class VarClass<T extends FieldType> {
    private readonly [classData]: VarClassData<T>;
    private constructor(data: VarClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(value: RefVar<T>): VarClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(value: VarClassData<T>): RefVar<T> {
        return new VarClass(value);
    }
}

type RefVar<T extends FieldType> = VarClass<T>;

type LiteralVar<T> = T extends Int
    ? number
    : T extends Bool
      ? boolean
      : T extends Set<infer TItem extends FieldType>
        ? Var<TItem>[]
        : T extends Object<infer TObj extends RecordType<FieldType>>
          ? {
                [K in keyof TObj]: Var<TObj[K]>;
            }
          : never;

interface TypeHolder<T> {
    type: T;
}

export type Var<T> = T extends FieldType
    ? RefVar<T> | LiteralVar<T>
    : T extends TypeHolder<infer TInner extends FieldType>
      ? Var<TInner>
      : never;

const x = ObjectClass.wrap({
    fields: {
        a1: IntClass.wrap({}),
        a2: IntClass.wrap({}),
    },
});

function f11(p: Var<typeof x>, o: "==", p2: Var<typeof x>) {
    console.log(p);
}

// f11(VariableWrapper.wrap({ values: x }));
f11(
    {
        a1: 1,
        a2: VarClass.wrap({ values: IntClass.wrap({}) }),
    },
    "==",
    VarClass.wrap({ values: x }),
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
    cmp: (a: Var<Int>, o: IntCmpOp, b: Var<Int>): Var<Bool> => {
        todo();
    },

    math: (a: Var<Int>, o: IntMathOp, b: Var<Int>): Var<Int> => {
        todo();
    },

    fold: (o: IntFoldOp, a: Var<Set<Int>>): Var<Int> => {
        todo();
    },

    sum: (...items: Var<Int>[]): Var<Int> => {
        todo();
    },

    product: (...items: Var<Int>[]): Var<Int> => {
        todo();
    },

    min: (...items: Var<Int>[]): Var<Int> => {
        todo();
    },

    max: (...items: Var<Int>[]): Var<Int> => {
        todo();
    },

    all: (o: IntAllOp, items: Var<Set<Int>>): Var<Bool> => {
        todo();
    },

    equal: (...items: Var<Int>[]): Var<Bool> => {
        todo();
    },

    none_equal: (...items: Var<Int>[]): Var<Bool> => {
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
        a: Var<Set<T>>,
        o: SetCmpOp,
        b: Var<Set<T>>,
    ): Var<Bool> => {
        todo();
    },

    element_of: <T extends FieldType>(
        a: Var<T>,
        o: "element of",
        b: Var<Set<T>>,
    ): Var<Bool> => {
        todo();
    },

    contains: <T extends FieldType>(
        a: Var<Set<T>>,
        o: "contains",
        b: Var<T>,
    ): Var<Bool> => {
        todo();
    },

    join: <T extends FieldType>(
        a: Var<Set<T>>,
        o: SetJoinOp,
        b: Var<Set<T>>,
    ): Var<Set<T>> => {
        todo();
    },

    fold: <T extends FieldType>(
        o: SetFoldOp,
        items: Var<Set<Set<T>>>,
    ): Var<Set<T>> => {
        todo();
    },

    union: <T extends FieldType>(...items: Var<Set<T>>[]): Var<Set<T>> => {
        todo();
    },

    intersect: <T extends FieldType>(...items: Var<Set<T>>[]): Var<Set<T>> => {
        todo();
    },

    disjunctive_union: <T extends FieldType>(
        ...items: Var<Set<T>>[]
    ): Var<Set<T>> => {
        todo();
    },

    all: <T extends FieldType>(
        o: SetAllOp,
        items: Var<Set<Set<T>>>,
    ): Var<Bool> => {
        todo();
    },

    equal: <T extends FieldType>(...items: Var<Set<T>>[]): Var<Bool> => {
        todo();
    },

    none_equal: <T extends FieldType>(...items: Var<Set<T>>[]): Var<Bool> => {
        todo();
    },

    all_disjoint: <T extends FieldType>(...items: Var<Set<T>>[]): Var<Bool> => {
        todo();
    },

    size: <T extends FieldType>(item: Var<Set<T>>): Var<Int> => {
        todo();
    },

    map: <T extends FieldType, R extends FieldType>(
        item: Var<Set<T>>,
        f: (value: Var<T>) => Var<R>,
    ): Var<Set<R>> => {
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
    cmp: (a: Var<Bool>, o: BoolCmpOp, b: Var<Bool>): Var<Bool> => {
        todo();
    },

    logic: (a: Var<Bool>, o: BoolLogicOp, b: Var<Bool>): Var<Bool> => {
        todo();
    },

    fold: (o: BoolSetOp, a: Var<Set<Bool>>): Var<Bool> => {
        todo();
    },

    all: (o: BoolAllOp, items: Var<Set<Bool>>): Var<Bool> => {
        todo();
    },

    equal: (...items: Var<Bool>[]): Var<Bool> => {
        todo();
    },

    none_equal: (...items: Var<Bool>[]): Var<Bool> => {
        todo();
    },

    none: (...items: Var<Bool>[]): Var<Bool> => {
        todo();
    },

    and: (...items: Var<Bool>[]): Var<Bool> => {
        todo();
    },

    or: (...items: Var<Bool>[]): Var<Bool> => {
        todo();
    },
};

// #endregion
// #region [Obj Op]

type ObjCmpOp = "==" | "!=";
type ObjAllOp = "==" | "!=";

const objOp = {
    cmp: <T extends RecordType<FieldType>>(
        a: Var<Object<T>>,
        o: ObjCmpOp,
        b: Var<Object<T>>,
    ): Var<Bool> => {
        todo();
    },

    all: <T extends RecordType<FieldType>>(
        o: ObjAllOp,
        items: Var<Set<Object<T>>>,
    ): Var<Bool> => {
        todo();
    },

    equal: <T extends RecordType<FieldType>>(
        ...items: Var<Object<T>>[]
    ): Var<Bool> => {
        todo();
    },

    none_equal: <T extends RecordType<FieldType>>(
        ...items: Var<Object<T>>[]
    ): Var<Bool> => {
        todo();
    },
};

// #endregion
// #region [Quantor Op]

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Matcher = any;

const quantorOp = {
    all: (f: (matcher: Matcher) => void): Var<Bool> => {
        todo();
    },
    exists: (f: (matcher: Matcher) => void): Var<Bool> => {
        todo();
    },
};

// #endregion
// #region [Op]

// #endregion

// #endregion
// #region [[Decl Value]]

interface DeclClassData<T extends FieldType> {
    values: T;
}
class DeclClass<T extends FieldType> {
    private readonly [classData]: DeclClassData<T>;
    private constructor(data: DeclClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(value: Decl<T>): DeclClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(value: DeclClassData<T>): Decl<T> {
        return new DeclClass(value);
    }
}

export type Decl<T extends FieldType> = DeclClass<T>;

// #endregion
// #region [[Test]]

// const field = {
//     decl: {
//         int: IntFieldDeclarationWrapper,
//         bool: BoolFieldDeclarationWrapper,
//         enum: EnumFieldDeclarationWrapper,
//         obj: ObjectFieldDeclarationWrapper,
//         arr: ArrayFieldDeclarationWrapper,
//     },
// } as const;

// const b1 = field.decl.obj.wrap({
//     fields: {
//         a: field.decl.int.wrap({}),
//         b: field.decl.obj.wrap({
//             fields: {
//                 c: field.decl.int.wrap({}),
//                 d: field.decl.int.wrap({}),
//                 e: field.decl.obj.wrap({
//                     fields: {
//                         f: field.decl.int.wrap({}),
//                         g: field.decl.int.wrap({}),
//                     },
//                 }),
//             },
//         }),
//         aa: field.decl.arr.wrap({
//             item: field.decl.int.wrap({}),
//         }),
//         ab: field.decl.arr.wrap({
//             item: field.decl.obj.wrap({
//                 fields: {
//                     a: field.decl.arr.wrap({
//                         item: field.decl.arr.wrap({
//                             item: field.decl.int.wrap({}),
//                         }),
//                     }),
//                 },
//             }),
//         }),
//         ac: field.decl.obj.wrap({
//             fields: {
//                 a: field.decl.enum.wrap({
//                     values: ["1", "2", "3"],
//                 }),
//             },
//         }),
//     },
// });

// const b2 = field.decl.obj.wrap({
//     fields: {
//         a: field.decl.enum.wrap({
//             values: ["1", "2", "3"],
//         }),
//         b: field.decl.bool.wrap({}),
//     },
// });

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
