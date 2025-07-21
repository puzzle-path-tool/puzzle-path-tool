import { classic_mod } from "../../classic_mod";
import { position } from "../../test_mod";
import { full_set } from "../deductions";

declare const quantor: any;
declare const set: any;
declare const int: any;
declare const obj: any;
declare const array: any;
type TODO = any;
const todo = "TODO";

export const column_rule = classic_mod.rule({
    name: "column_rule",
    data: {
        cells: array.field(position()),
        values: array.field(int.field),
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "column_to_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const column1 = matcher.pool.get_one(column_rule);

        emitter.emit(full_set, [
            { cells: column1.cells, values: column1.values },
        ]);
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher: TODO, emitter: TODO) => {
        const column = matcher.pool.get_one(column_rule);

        emitter.resolve(column);
    },
});
