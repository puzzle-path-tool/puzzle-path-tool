import type { ExamplePuzzptApi } from "./puzzpt_api.generated";

export function do_stuff(param: ExamplePuzzptApi): string {
    return "";
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
