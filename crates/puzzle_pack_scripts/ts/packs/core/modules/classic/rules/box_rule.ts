import { classic_mod } from "../../classic_mod";
import { position } from "../../test_mod";
import { full_set } from "../deductions";
import { column_rule } from "./column_rule";

declare const quantor: any;
declare const set: any;
declare const int: any;
declare const obj: any;
declare const array: any;
type TODO = any;
const todo = "TODO";

export const box_rule = classic_mod.rule({
    name: "box_rule",
    data: {
        cells: array.field(position()),
        values: array.field(int.field),
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "box_to_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const box1 = matcher.pool.get_one(box_rule);

        emitter.emit(full_set, [{ cells: box1.cells, values: box1.values }]);
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher: TODO, emitter: TODO) => {
        const box = matcher.pool.get_one(box_rule);

        emitter.resolve(box);
    },
});
