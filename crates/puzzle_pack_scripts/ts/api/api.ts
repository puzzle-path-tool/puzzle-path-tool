// #region [[Definitions]]

const classData = Symbol("classData");
const puzzptExport = Symbol.for("puzzpt_export");

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
// #region [[Namespace]]

interface InfoParams {
    doc: string;
}

interface NamespaceInfoParams {
    name: string;
    description?: string;
}

interface NamespaceCreateParams<
    TType extends FieldType | undefined,
    TOp extends RecordType | undefined,
> {
    decl?: TType extends FieldType ? Decl<TType> : undefined;
    op?: TOp;
}

interface NamespaceParams<
    TType extends FieldType | undefined,
    TOp extends RecordType | undefined,
> extends NamespaceInfoParams {
    create?: () => NamespaceCreateParams<TType, TOp>;
}

interface NamespaceInternalCreateParams<
    TOp extends RecordType | undefined,
    TDecl,
    TType,
> {
    op?: TOp;
    decl?: TDecl;
    t?: TType;
}

interface NamespaceInternalParams<
    TOp extends RecordType | undefined,
    TDecl,
    TType,
> extends NamespaceInfoParams {
    create?: () => NamespaceInternalCreateParams<TOp, TDecl, TType>;
}

type NamespaceFromParams<
    TType extends FieldType | undefined,
    TOp extends RecordType | undefined,
> = Namespace<
    TOp,
    TType extends FieldType ? (info?: InfoParams) => Decl<TType> : undefined,
    TType
>;

type NamespaceFieldsFromParams<
    TType extends FieldType | undefined,
    TOp extends RecordType | undefined,
> = NamespaceFields<
    TOp,
    TType extends FieldType ? (info?: InfoParams) => Decl<TType> : undefined,
    TType
>;

type NamespaceClassDataFromParams<
    TType extends FieldType | undefined,
    TOp extends RecordType | undefined,
> = NamespaceClassData<
    TOp,
    TType extends FieldType ? (info?: InfoParams) => Decl<TType> : undefined,
    TType
>;

type NamespaceFields<
    TOp extends RecordType | undefined,
    TDecl,
    TType,
> = RecordType &
    (TOp extends RecordType ? { readonly op: TOp } : unknown) &
    (TDecl extends undefined ? unknown : { readonly decl: TDecl }) &
    (TType extends undefined ? unknown : { readonly t: TType });

declare const y23: NamespaceFields<{ x: number }, number, string>;

function wrapNamespaceFields<
    const TOp extends RecordType | undefined,
    const TDecl,
    const TType,
>(
    values: NamespaceClassData<TOp, TDecl, TType>,
): NamespaceFields<TOp, TDecl, TType> {
    // return mapFields<T, BFields<T>>(obj, (key, value) => [
    //     [
    //         key,
    //         {
    //             id: id,
    //             value: value,
    //         },
    //     ],
    // ]);
    todo();
}

interface NamespaceClassData<TOp extends RecordType | undefined, TDecl, TType> {
    name: string;
    description?: string;
    module: Mod;
    fields: NamespaceFields<TOp, TDecl, TType>;
}
class NamespaceClass<TOp extends RecordType | undefined, TDecl, TType> {
    private readonly [classData]: NamespaceClassData<TOp, TDecl, TType>;
    private constructor(data: NamespaceClassData<TOp, TDecl, TType>) {
        this[classData] = data;
    }
    static unwrap<TOp extends RecordType | undefined, TDecl, TType>(
        value: Namespace<TOp, TDecl, TType>,
    ): NamespaceClassData<TOp, TDecl, TType> {
        return value[classData];
    }
    static wrap<
        const TOp extends RecordType | undefined = undefined,
        const TDecl = undefined,
        const TType = undefined,
    >(
        value: NamespaceClassData<TOp, TDecl, TType>,
    ): Namespace<TOp, TDecl, TType> {
        return wrapProxy(new NamespaceClass(value), wrapNamespaceFields(value));
    }
    static create<
        const TType extends FieldType | undefined = undefined,
        const TOp extends RecordType | undefined = undefined,
    >(
        params: NamespaceParams<TType, TOp>,
        module: Mod,
    ): NamespaceFromParams<TType, TOp> {
        todo();
    }
    static createInternal<
        const TOp extends RecordType | undefined = undefined,
        const TDecl = undefined,
        const TType = undefined,
    >(
        params: NamespaceInternalParams<TOp, TDecl, TType>,
        module: Mod,
    ): Namespace<TOp, TDecl, TType> {
        todo();
    }
}

export type Namespace<
    TOp extends RecordType | undefined,
    TDecl,
    TType,
> = NamespaceClass<TOp, TDecl, TType> & NamespaceFields<TOp, TDecl, TType>;

declare const fieldF: Decl<Obj<{ x: Int }>>;
declare const modF: Mod;

// #endregion
// #region [[Field Types]]

// #region [Field Type]

/* eslint-disable @typescript-eslint/no-explicit-any */
export type FieldType = Int | Bool | Enum<any> | Obj<any> | Set<any>;
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
    static isObj(value: unknown): value is Obj<RecordType<FieldType>> {
        return value instanceof ObjClass;
    }
    static isSet(value: unknown): value is Set<FieldType> {
        return value instanceof SetClass;
    }
}

// #endregion
// #region [Int Field Declaration]

class IntClass {
    private readonly [classData]: undefined = undefined;
    private constructor() {}
    static readonly instance = new IntClass();
}
export type Int = IntClass;

// #endregion
// #region [Bool Field Declaration]

class BoolClass {
    private readonly [classData]: undefined = undefined;
    private constructor() {}
    static readonly instance = new BoolClass();
}
export type Bool = BoolClass;

// #endregion
// #region [Enum Field Declaration]

interface EnumClassData<T extends readonly string[]> {
    variants: T;
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

type ObjFields<T extends RecordType<FieldType>> = Readonly<{
    [K in keyof T as K extends string ? K : never]: T[K];
}>;

function wrapObjFields<const T extends RecordType<FieldType>>(
    values: ObjClassData<T>,
): ObjFields<T> {
    // return mapFields<T, BFields<T>>(obj, (key, value) => [
    //     [
    //         key,
    //         {
    //             id: id,
    //             value: value,
    //         },
    //     ],
    // ]);
    todo();
}

interface ObjClassData<T extends RecordType<FieldType>> {
    fields: T;
}
class ObjClass<T extends RecordType<FieldType>> {
    private readonly [classData]: ObjClassData<T>;
    private constructor(data: ObjClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends RecordType<FieldType>>(
        value: Obj<T>,
    ): ObjClassData<T> {
        return value[classData];
    }
    static wrap<const T extends RecordType<FieldType>>(
        value: ObjClassData<T>,
    ): Obj<T> {
        return wrapProxy(new ObjClass(value), wrapObjFields(value));
    }
}
export type Obj<T extends RecordType<FieldType>> = ObjClass<T> & ObjFields<T>;

// #endregion
// #region [Set Field Declaration]

interface SetClassData<T extends FieldType> {
    itemFieldType: T;
}
class SetClass<T extends FieldType> {
    private readonly [classData]: SetClassData<T>;
    private constructor(data: SetClassData<T>) {
        this[classData] = data;
        this.item = data.itemFieldType;
    }
    static unwrap<T extends FieldType>(value: Set<T>): SetClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(value: SetClassData<T>): Set<T> {
        return new SetClass(value);
    }
    readonly item: T;
}
export type Set<T extends FieldType> = SetClass<T>;

// #endregion

// #endregion
// #region [[Decl Value]]

interface DeclClassData<T extends FieldType> {
    fieldType: T;
}
class DeclClass<T extends FieldType> {
    private readonly [classData]: DeclClassData<T>;
    private constructor(data: DeclClassData<T>) {
        this[classData] = data;
        this.t = data.fieldType;
    }
    static unwrap<T extends FieldType>(value: Decl<T>): DeclClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(value: DeclClassData<T>): Decl<T> {
        return new DeclClass(value);
    }
    readonly t: T;
}

export type Decl<T extends FieldType> = DeclClass<T>;

// #endregion
// #region [[Modules]]

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type NoArray<T> = T extends any[] ? never : T;

function unifyIntoArray<T>(value: NoArray<T> | T[]): T[] {
    if (Array.isArray(value)) {
        return value;
    } else {
        return [value];
    }
}

export const pack = {
    /**
     * Create a new pack, must be exported to be loaded.
     */
    create: (params: PackParams): Pack => {
        return PackClass.wrap({
            name: params.name,
            authors: unifyIntoArray(params.authors),
            description: params.description,
        });
    },
} as const;

interface PackParams {
    /**
     * The name of the pack.
     *
     * Used as an identifier, when combined with the authors.
     *
     * Must be unique, when combined with authors.
     */
    name: string;
    /**
     * List of authors.
     *
     * Are used as part of the identifier.
     */
    authors: string | string[];
    /**
     * A short description of the pack.
     */
    description: string;
}
interface PackClassData {
    name: string;
    authors: string[];
    description: string;
}
class PackClass {
    private readonly [classData]: PackClassData;
    private readonly [puzzptExport]: TODO;
    private constructor(data: PackClassData) {
        this[classData] = data;
    }
    static unwrap(value: Pack): PackClassData {
        return value[classData];
    }
    static wrap(value: PackClassData): Pack {
        return new PackClass(value);
    }
    /**
     * Create a new module.
     */
    module(params: ModParams): Mod {
        return ModClass.wrap({
            pack: this,
            parents: [],
            name: params.name,
        });
    }
}

export type Pack = PackClass;

interface ModParams {
    /**
     * The name of the module.
     *
     * Used as an identifier, when combined with parents.
     *
     * Must be unique, relative to its parent.
     */
    name: string;
}

interface ModClassData {
    pack: Pack;
    parents: Mod[];
    name: string;
}
class ModClass {
    private readonly [classData]: ModClassData;
    private constructor(data: ModClassData) {
        this[classData] = data;
    }
    static unwrap(value: Mod): ModClassData {
        return value[classData];
    }
    static wrap(value: ModClassData): Mod {
        return new ModClass(value);
    }
    static create(params: ModParams, parent: Mod): Mod {
        const data = ModClass.unwrap(parent);

        return ModClass.wrap({
            pack: data.pack,
            parents: [...data.parents, parent],
            name: params.name,
        });
    }

    /**
     * Create a new submodule.
     */
    submodule(params: ModParams): Mod {
        return ModClass.create(params, this);
    }
    /**
     * Create a new namespace.
     */
    namespace<
        TType extends FieldType | undefined = undefined,
        TOp extends RecordType | undefined = undefined,
    >(params: NamespaceParams<TType, TOp>): NamespaceFromParams<TType, TOp> {
        return NamespaceClass.create(params, this);
    }
    /**
     * Create a new rule, must be exported to be loaded.
     */
    rule(params: RuleParams): Rule {
        return RuleClass.create(params);
    }
    /**
     * Create a new deduction, must be exported to be loaded.
     */
    deduction<
        const TType extends FieldType,
        const TOp extends RecordType | undefined = undefined,
    >(params: DeductionParams<TType, TOp>): Deduction<TType, TOp> {
        return DeductionClass.create(params, this);
    }
    /**
     * Create a new logic step, must be exported to be loaded.
     */
    step(params: LogicStepParams): LogicStep {
        return LogicStepClass.create(params);
    }
}

export type Mod = ModClass;

const apiPack = pack.create({
    name: "api",
    authors: "puzzpt",
    description: "Builtin Api Pack",
});
const apiMod = apiPack.module({
    name: "api",
});

// #endregion
// #region [[Var Value]]

// #region [Variable]
type RefVarFields<T extends FieldType> = Readonly<
    T extends Obj<infer R extends RecordType<FieldType>>
        ? {
              [K in keyof R as K extends string ? K : never]: Var<R[K]>;
          }
        : RecordType
>;

function wrapRefVarFields<const T extends FieldType>(
    values: RefVarClassData<T>,
): RefVarFields<T> {
    // return mapFields<T, BFields<T>>(obj, (key, value) => [
    //     [
    //         key,
    //         {
    //             id: id,
    //             value: value,
    //         },
    //     ],
    // ]);
    todo();
}

interface RefVarClassData<T extends FieldType> {
    fieldType: T;
}
class RefVarClass<T extends FieldType> {
    private readonly [classData]: RefVarClassData<T>;
    private constructor(data: RefVarClassData<T>) {
        this[classData] = data;
    }
    static unwrap<T extends FieldType>(value: RefVar<T>): RefVarClassData<T> {
        return value[classData];
    }
    static wrap<const T extends FieldType>(
        value: RefVarClassData<T>,
    ): RefVar<T> {
        return wrapProxy(new RefVarClass(value), wrapRefVarFields(value));
    }
}

type RefVar<T extends FieldType> = RefVarClass<T> & RefVarFields<T>;

type CompositeVar<T> = T extends Int
    ? number
    : T extends Bool
      ? boolean
      : T extends Enum<infer TVariants extends readonly string[]>
        ? TVariants[number]
        : T extends Set<infer TItem extends FieldType>
          ? Var<TItem>[]
          : T extends Obj<infer TObj extends RecordType<FieldType>>
            ? {
                  [K in keyof TObj]: Var<TObj[K]>;
              }
            : never;

export type LiteralVar<T> = T extends Int
    ? number
    : T extends Bool
      ? boolean
      : T extends Enum<infer TVariants extends readonly string[]>
        ? TVariants[number]
        : T extends Set<infer TItem extends FieldType>
          ? LiteralVar<TItem>[]
          : T extends Obj<infer TObj extends RecordType<FieldType>>
            ? {
                  [K in keyof TObj]: LiteralVar<TObj[K]>;
              }
            : never;

export type Var<T extends FieldType> = RefVar<T> | CompositeVar<T>;

const x = ObjClass.wrap({
    fields: {
        a1: IntClass.instance,
        a2: IntClass.instance,
    },
});

function f11(p: Var<typeof x>, o: "==", p2: Var<typeof x>) {
    console.log(p);
}

// f11(VariableWrapper.wrap({ values: x }));
f11(
    {
        a1: 1,
        a2: RefVarClass.wrap({ fieldType: IntClass.instance }),
    },
    "==",
    RefVarClass.wrap({ fieldType: x }),
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
type IntAllOp =
    | "=="
    | "!="
    | "> 0"
    | ">= 0"
    | "< 0"
    | "<= 0"
    | "== 0"
    | "!= 0"
    | "prime";

export const int = apiMod.namespace({
    name: "int",
    create: () => ({
        decl: DeclClass.wrap({
            fieldType: IntClass.instance,
        }),
        op: {
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

            is_prime: (...items: Var<Int>[]): Var<Bool> => {
                todo();
            },
        },
    }),
});

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

export const set = NamespaceClass.createInternal(
    {
        name: "set",
        create: () => ({
            decl: <const T extends FieldType>(
                item: Decl<T>,
                info?: InfoParams,
            ): Decl<Set<T>> => {
                todo();
            },
            op: {
                cmp: <const T extends FieldType>(
                    a: Var<Set<T>>,
                    o: SetCmpOp,
                    b: Var<Set<T>>,
                ): Var<Bool> => {
                    todo();
                },

                element_of: <const T extends FieldType>(
                    a: Var<T>,
                    o: "element of",
                    b: Var<Set<T>>,
                ): Var<Bool> => {
                    todo();
                },

                contains: <const T extends FieldType>(
                    a: Var<Set<T>>,
                    o: "contains",
                    b: Var<T>,
                ): Var<Bool> => {
                    todo();
                },

                join: <const T extends FieldType>(
                    a: Var<Set<T>>,
                    o: SetJoinOp,
                    b: Var<Set<T>>,
                ): Var<Set<T>> => {
                    todo();
                },

                fold: <const T extends FieldType>(
                    o: SetFoldOp,
                    items: Var<Set<Set<T>>>,
                ): Var<Set<T>> => {
                    todo();
                },

                union: <const T extends FieldType>(
                    ...items: Var<Set<T>>[]
                ): Var<Set<T>> => {
                    todo();
                },

                intersect: <const T extends FieldType>(
                    ...items: Var<Set<T>>[]
                ): Var<Set<T>> => {
                    todo();
                },

                disjunctive_union: <const T extends FieldType>(
                    ...items: Var<Set<T>>[]
                ): Var<Set<T>> => {
                    todo();
                },

                all: <const T extends FieldType>(
                    o: SetAllOp,
                    items: Var<Set<Set<T>>>,
                ): Var<Bool> => {
                    todo();
                },

                all_disjoint: <const T extends FieldType>(
                    ...items: Var<Set<T>>[]
                ): Var<Bool> => {
                    todo();
                },

                size: <const T extends FieldType>(
                    item: Var<Set<T>>,
                ): Var<Int> => {
                    todo();
                },

                map: <const T extends FieldType, R extends FieldType>(
                    item: Var<Set<T>>,
                    f: (value: Var<T>) => Var<R>,
                ): Var<Set<R>> => {
                    todo();
                },
            },
        }),
    },
    apiMod,
);

// #endregion
// #region [Bool Op]

type BoolCmpOp = "==" | "!=";
type BoolLogicOp = "or" | "and" | "xor" | "nor" | "nand" | "xnor";
type BoolSetOp = "all" | "any" | "none" | BoolCmpOp | BoolLogicOp;
type BoolAllOp = "==" | "!=" | "true" | "false";

export const bool = apiMod.namespace({
    name: "bool",
    create: () => ({
        decl: DeclClass.wrap({
            fieldType: BoolClass.instance,
        }),
        op: {
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

            none: (...items: Var<Bool>[]): Var<Bool> => {
                todo();
            },

            and: (...items: Var<Bool>[]): Var<Bool> => {
                todo();
            },

            or: (...items: Var<Bool>[]): Var<Bool> => {
                todo();
            },
        },
    }),
});

// #endregion
// #region [Table Op]

interface TableClassData<TA extends FieldType, TB extends FieldType> {
    a: TA;
    b: TB;
    mappings: [LiteralVar<TA>, LiteralVar<TB>][];
}
class TableClass<TA extends FieldType, TB extends FieldType> {
    private readonly [classData]: TableClassData<TA, TB>;
    private constructor(data: TableClassData<TA, TB>) {
        this[classData] = data;
    }
    static unwrap<TA extends FieldType, TB extends FieldType>(
        value: Table<TA, TB>,
    ): TableClassData<TA, TB> {
        return value[classData];
    }
    static wrap<const TA extends FieldType, const TB extends FieldType>(
        value: TableClassData<TA, TB>,
    ): Table<TA, TB> {
        return new TableClass(value);
    }
}

const l1 = RefVarClass.wrap({
    fieldType: IntClass.instance,
});

const l2 = RefVarClass.wrap({
    fieldType: IntClass.instance,
});

const l3 = RefVarClass.wrap({
    fieldType: EnumClass.wrap({ variants: ["A", "B"] }),
});

// objOp.equal(l2, 1);
// objOp.equal(l3, "B");

const csacas = TableClass.wrap({
    a: EnumClass.wrap({ variants: ["A", "B"] }),
    b: IntClass.instance,
    mappings: [
        ["A", 4],
        ["B", 5],
    ],
});

const csacas2 = TableClass.wrap({
    a: EnumClass.wrap({ variants: ["A", "B"] }),
    b: ObjClass.wrap({
        fields: {
            x: IntClass.instance,
        },
    }),
    mappings: [
        ["A", { x: 3 }],
        ["B", { x: 4 }],
    ],
});

export type Table<TA extends FieldType, TB extends FieldType> = TableClass<
    TA,
    TB
>;

export const table = apiMod.namespace({
    name: "table",
    create: () => ({
        op: {
            forwards: <const TA extends FieldType, const TB extends FieldType>(
                item: Var<TA>,
                o: "via",
                table: Table<TA, TB>,
            ): Var<TB> => {
                todo();
            },

            backwards: <const TA extends FieldType, const TB extends FieldType>(
                item: Var<TB>,
                o: "via",
                table: Table<TA, TB>,
            ): Var<TA> => {
                todo();
            },
        },
    }),
});

// #endregion
// #region [Obj Op]

type ObjCmpOp = "==" | "!=";
type ObjAllOp = "==" | "!=";

type DeclObjOf<T extends RecordType<FieldType>> = {
    [K in keyof T]: Decl<T[K]>;
};

export const obj = NamespaceClass.createInternal(
    {
        name: "obj",
        create: () => ({
            decl: <const T extends RecordType<FieldType>>(
                fields: DeclObjOf<T>,
                info?: InfoParams,
            ): Decl<Obj<T>> => {
                todo();
            },
            op: {
                cmp: <const T extends FieldType>(
                    a: Var<T>,
                    o: ObjCmpOp,
                    b: Var<T>,
                ): Var<Bool> => {
                    todo();
                },

                all: <const T extends FieldType>(
                    o: ObjAllOp,
                    items: Var<Set<T>>,
                ): Var<Bool> => {
                    todo();
                },

                equal: <const T extends FieldType>(
                    ...items: Var<T>[]
                ): Var<Bool> => {
                    todo();
                },

                none_equal: <const T extends FieldType>(
                    ...items: Var<T>[]
                ): Var<Bool> => {
                    todo();
                },
            },
        }),
    },
    apiMod,
);

// #endregion
// #region [Quantor Op]

interface MatcherClassData {
    name: string;
}

class MatcherClass {
    // private readonly [classData]: MatcherClassData;
    // private constructor(data: RefVarClassData<T>) {
    //     this[classData] = data;
    // }
    // static unwrap<T extends FieldType>(value: RefVar<T>): RefVarClassData<T> {
    //     return value[classData];
    // }
    readonly pool = {
        get_one: <const T extends FieldType>(
            deduction: DeductionVar<T>,
        ): Var<T> => {
            const item = this.get_one(deduction.t);
            this.where(pool.op.one(item, "from", deduction));
            return item;
        },
        get_many: <const T extends FieldType>(
            deduction: DeductionVar<T>,
        ): Var<Set<T>> => {
            const items = this.get_many(deduction.t);
            this.where(pool.op.many(items, "from", deduction));
            return items;
        },
    };
    get_one<const T extends FieldType>(fieldType: T): Var<T> {
        todo();
    }
    get_many<const T extends FieldType>(fieldType: T): Var<Set<T>> {
        todo();
    }
    debugSymbols<const T extends RecordType>(symbols: T) {}
    where(condition: Var<Bool>) {}
    require(condition: Var<Bool>) {}
}

export type Matcher = MatcherClass;

class EmitterClass {
    emit_one<
        const TType extends FieldType,
        const TOp extends RecordType | undefined,
    >(deduction: Deduction<TType, TOp>, value: Var<TType>) {}
    emit_error(description: string) {}
}

export type Emitter = EmitterClass;

export const quantor = apiMod.namespace({
    name: "quantor",
    create: () => ({
        op: {
            all: (f: (matcher: Matcher) => void): Var<Bool> => {
                todo();
            },
            exists: (f: (matcher: Matcher) => void): Var<Bool> => {
                todo();
            },
        },
    }),
});

// #endregion
// #region [Pool Op]

export const pool = apiMod.namespace({
    name: "pool",
    create: () => ({
        op: {
            one: <const T extends FieldType>(
                a: Var<T>,
                o: "from",
                b: DeductionVar<T>,
            ): Var<Bool> => {
                todo();
            },

            many: <const T extends FieldType>(
                a: Var<Set<T>>,
                o: "from",
                b: DeductionVar<T>,
            ): Var<Bool> => {
                todo();
            },
        },
    }),
});

// #endregion
// #region [Op]

// #endregion

// #endregion
// #region [[Export]]

interface RuleParams {
    name: string;
}

interface RuleClassData {
    name: string;
}
class RuleClass {
    private readonly [classData]: RuleClassData;
    private readonly [puzzptExport]: TODO;
    private constructor(data: RuleClassData) {
        this[classData] = data;
    }
    static unwrap(value: Rule): RuleClassData {
        return value[classData];
    }
    static wrap(value: RuleClassData): Rule {
        return new RuleClass(value);
    }
    static create(params: RuleParams): Rule {
        todo();
    }
}

export type Rule = RuleClass;

type DeductionFields<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> = NamespaceFieldsFromParams<TType, TOp>;

interface DeductionCreateParams<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> {
    decl: Decl<TType>;
    op?: TOp;
}

interface DeductionParams<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> extends NamespaceInfoParams {
    create?: () => DeductionCreateParams<TType, TOp>;
}

interface DeductionClassData<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> {
    name: string;
    module: Mod;
    namespace: NamespaceClassDataFromParams<TType, TOp>;
}
class DeductionClass<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> {
    private readonly [classData]: DeductionClassData<TType, TOp>;
    private readonly [puzzptExport]: TODO;
    private constructor(data: DeductionClassData<TType, TOp>) {
        this[classData] = data;
    }
    static unwrap<TType extends FieldType, TOp extends RecordType | undefined>(
        value: Deduction<TType, TOp>,
    ): DeductionClassData<TType, TOp> {
        return value[classData];
    }
    static wrap<
        const TType extends FieldType,
        const TOp extends RecordType | undefined = undefined,
    >(value: DeductionClassData<TType, TOp>): Deduction<TType, TOp> {
        return wrapProxy(
            new DeductionClass(value),
            wrapNamespaceFields(value.namespace),
        );
    }
    static create<
        const TType extends FieldType,
        const TOp extends RecordType | undefined = undefined,
    >(params: DeductionParams<TType, TOp>, module: Mod): Deduction<TType, TOp> {
        todo();
    }
    readonly t!: TType;
}

export type Deduction<
    TType extends FieldType,
    TOp extends RecordType | undefined,
> = DeductionClass<TType, TOp> & DeductionFields<TType, TOp>;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type DeductionVar<T extends FieldType> = Deduction<T, any>;

interface LogicStepParams {
    name: string;
    logic: (matcher: Matcher, emitter: Emitter) => void;
}

interface LogicStepClassData {
    name: string;
}
class LogicStepClass {
    private readonly [classData]: LogicStepClassData;
    private readonly [puzzptExport]: TODO;
    private constructor(data: LogicStepClassData) {
        this[classData] = data;
    }
    static unwrap(value: LogicStep): LogicStepClassData {
        return value[classData];
    }
    static wrap(value: LogicStepClassData): LogicStep {
        return new LogicStepClass(value);
    }
    static create(params: LogicStepParams): LogicStep {
        todo();
    }
}

export type LogicStep = LogicStepClass;

// #endregion
// #region [[Test]]

// #endregion
