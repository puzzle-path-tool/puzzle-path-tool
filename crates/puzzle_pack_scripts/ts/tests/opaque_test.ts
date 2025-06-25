// Opaque Data test

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

// Opaque Data test 2

type RecordType<T = unknown> = Record<string, T>;

type BData<T extends RecordType> = { name: string; fields: T };
type BFields<T extends RecordType> = {
    [K in keyof T]: { id: string; value: T[K] };
};

function wrapProxyDyn<TBase extends object, TExtra extends RecordType>(
    base: TBase,
    extra: (target: TBase) => TExtra,
): TBase & TExtra {
    return new Proxy(base, {
        get(target, prop, receiver) {
            const extraObj = extra(target);

            if (
                typeof prop === "string" &&
                prop in extraObj &&
                prop != "_private_data"
            ) {
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

class BWrapper<T extends RecordType> {
    private readonly _private_data: BData<T>;
    private constructor(data: BData<T>) {
        this._private_data = data;
    }
    static unwrap<T extends RecordType>(value: B<T>): BData<T> {
        return value._private_data;
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

// Field Declarations

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

// Int Field Declaration

type IntFieldDeclarationData = object;
class IntFieldDeclarationWrapper {
    private readonly _private_data: IntFieldDeclarationData;
    private constructor(data: IntFieldDeclarationData) {
        this._private_data = data;
    }
    static unwrap(value: IntFieldDeclaration): IntFieldDeclarationData {
        return value._private_data;
    }
    static wrap(value: IntFieldDeclarationData): IntFieldDeclaration {
        return new IntFieldDeclarationWrapper(value);
    }
}
export type IntFieldDeclaration = IntFieldDeclarationWrapper;

// Bool Field Declaration

type BoolFieldDeclarationData = object;
class BoolFieldDeclarationWrapper {
    private readonly _private_data: BoolFieldDeclarationData;
    private constructor(data: BoolFieldDeclarationData) {
        this._private_data = data;
    }
    static unwrap(value: BoolFieldDeclaration): BoolFieldDeclarationData {
        return value._private_data;
    }
    static wrap(value: BoolFieldDeclarationData): BoolFieldDeclaration {
        return new BoolFieldDeclarationWrapper(value);
    }
}
export type BoolFieldDeclaration = BoolFieldDeclarationWrapper;

// Enum Field Declaration

interface EnumFieldDeclarationData<T extends readonly string[]> {
    values: T;
}
class EnumFieldDeclarationWrapper<T extends readonly string[]> {
    private readonly _private_data: EnumFieldDeclarationData<T>;
    private constructor(data: EnumFieldDeclarationData<T>) {
        this._private_data = data;
    }
    static unwrap<T extends readonly string[]>(
        value: EnumFieldDeclaration<T>,
    ): EnumFieldDeclarationData<T> {
        return value._private_data;
    }
    static wrap<const T extends readonly string[]>(
        value: EnumFieldDeclarationData<T>,
    ): EnumFieldDeclaration<T> {
        return new EnumFieldDeclarationWrapper(value);
    }
}

export type EnumFieldDeclaration<T extends readonly string[]> =
    EnumFieldDeclarationWrapper<T>;

// Object Field Declaration

interface ObjectFieldDeclarationData<T extends RecordType<FieldDeclaration>> {
    fields: T;
}
class ObjectFieldDeclarationWrapper<T extends RecordType<FieldDeclaration>> {
    private readonly _private_data: ObjectFieldDeclarationData<T>;
    private constructor(data: ObjectFieldDeclarationData<T>) {
        this._private_data = data;
    }
    static unwrap<T extends RecordType<FieldDeclaration>>(
        value: ObjectFieldDeclaration<T>,
    ): ObjectFieldDeclarationData<T> {
        return value._private_data;
    }
    static wrap<const T extends RecordType<FieldDeclaration>>(
        value: ObjectFieldDeclarationData<T>,
    ): ObjectFieldDeclaration<T> {
        return new ObjectFieldDeclarationWrapper(value);
    }
}
export type ObjectFieldDeclaration<T extends RecordType<FieldDeclaration>> =
    ObjectFieldDeclarationWrapper<T>;

// Array Field Declaration

interface ArrayFieldDeclarationData<T extends FieldDeclaration> {
    item: T;
}
class ArrayFieldDeclarationWrapper<T extends FieldDeclaration> {
    private readonly _private_data: ArrayFieldDeclarationData<T>;
    private constructor(data: ArrayFieldDeclarationData<T>) {
        this._private_data = data;
    }
    static unwrap<T extends FieldDeclaration>(
        value: ArrayFieldDeclaration<T>,
    ): ArrayFieldDeclarationData<T> {
        return value._private_data;
    }
    static wrap<const T extends FieldDeclaration>(
        value: ArrayFieldDeclarationData<T>,
    ): ArrayFieldDeclaration<T> {
        return new ArrayFieldDeclarationWrapper(value);
    }
}
export type ArrayFieldDeclaration<T extends FieldDeclaration> =
    ArrayFieldDeclarationWrapper<T>;

// Test

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
