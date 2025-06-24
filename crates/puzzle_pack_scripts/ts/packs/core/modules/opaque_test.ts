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

type RecordType = Record<string, unknown>;

type BData<T extends RecordType> = { name: string; fields: T };
type BFields<T extends RecordType> = {
    [K in keyof T]: { id: string; value: T[K] };
};

function wrapProxy<TBase extends object, TExtra extends RecordType>(
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

function wrapFields<const T extends RecordType>(
    obj: T,
    id: string,
): BFields<T> {
    const x = Object.entries(obj).map(([key, value]) => {
        const oldValue = value as T[keyof T];

        const newValue: BFields<T>[keyof BFields<T>] = {
            id: id,
            value: oldValue,
        };

        return [key, newValue];
    });

    return Object.fromEntries(x) as BFields<T>;
}

class BWrapper<T extends RecordType> {
    readonly #data: BData<T>;
    private constructor(data: BData<T>) {
        this.#data = data;
    }
    static unwrap<T extends RecordType>(value: B<T>): BData<T> {
        return value.#data;
    }
    static wrap<const T extends RecordType>(value: BData<T>): B<T> {
        return wrapProxy(new BWrapper(value), (target) => {
            return wrapFields(target.#data.fields, target.#data.name);
        });
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

/*
Run in Console:

npm run check; node out/packs/core/modules/opaque_test.js

*/
