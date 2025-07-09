import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    cell_value,
    full_set,
    non_repeat_set,
    required_value,
} from "./deductions";

declare const quantor: any;
declare const set: any;
declare const int: any;
declare const obj: any;
type TODO = any;
const todo = "TODO";

const no_two_cell_values = classic_mod.step({
    name: "no_two_cell_values",
    logic: (matcher: TODO, emitter: TODO) => {
        const cell_value1 = matcher.pool.get_one(cell_value);
        const cell_value2 = matcher.pool.get_one(cell_value);

        matcher.where(obj.op.cmp(cell_value1.cell, "==", cell_value2.cell));
        matcher.where(int.op.cmp(cell_value1.value, "!=", cell_value2.value));

        emitter.error();
    },
});

const no_zero_allowed_values = classic_mod.step({
    name: "no_zero_allowed_values",
    logic: (matcher: TODO, emitter: TODO) => {
        const allowed_values1 = matcher.pool.get_one(allowed_values);

        matcher.where(obj.op.cmp(int.op.size(allowed_values1.values), "==", 0));

        emitter.error();
    },
});

const no_zero_required_value_cells = classic_mod.step({
    name: "no_zero_required_value_cells",
    logic: (matcher: TODO, emitter: TODO) => {
        const required_value1 = matcher.pool.get_one(required_value);

        matcher.where(obj.op.cmp(int.op.size(required_value1.cells), "==", 0));

        emitter.error();
    },
});

const no_insufficient_split = classic_mod.step({
    name: "no_insufficient_split",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(non_repeat_set);

        const allowed_values_set = matcher.pool.get_many(allowed_values);
        matcher.where(
            set.do(
                set.op.map(allowed_values_set, (x: TODO) => {
                    return x.cell;
                }),
                "subset of",
                set1.cells,
            ),
        );
        const values = set.op.union(
            set.op.map(allowed_values_set, (x: TODO) => {
                return x.values;
            }),
        );
        matcher.require(int.op.cmp(values.size, "<", allowed_values_set.size));

        emitter.error();
    },
});

const no_repeat_in_non_repeat = classic_mod.step({
    name: "no_repeat_in_non_repeat",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(non_repeat_set);

        const required_value1 = matcher.pool.get_one(required_value);
        const required_value2 = matcher.pool.get_one(required_value);

        matcher.where(set.do(required_value1.cells, "subset of", set1.cells));
        matcher.where(set.do(required_value2.cells, "subset of", set1.cells));
        matcher.where(
            int.op.cmp(required_value1.value, "==", required_value2.value),
        );
        matcher.require(
            int.op.cmp(
                set.do(
                    required_value1.cells,
                    "intersect",
                    required_value2.cells,
                ),
                "==",
                0,
            ),
        );

        emitter.error();
    },
});

const all_values_present = classic_mod.step({
    name: "all_values_present",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(non_repeat_set);

        const allowed_values_set = matcher.pool.get_many(allowed_values);

        matcher.where(
            obj.op.cmp(
                set.op.map(allowed_values_set, (x: TODO) => {
                    return x.cell;
                }),
                "==",
                set1.cells,
            ),
        );
        matcher.require(
            set.do(
                set1.values,
                "subset of",
                set.op.union(
                    set.op.map(allowed_values_set, (x: TODO) => {
                        return x.values;
                    }),
                ),
            ),
        );

        emitter.error();
    },
});
