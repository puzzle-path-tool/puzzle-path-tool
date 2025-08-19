import { int, obj, set } from "api/api";
import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    cell_value,
    full_set,
    non_repeat_set,
    required_value,
} from "./deductions";

const no_two_cell_values = classic_mod.step({
    name: "no_two_cell_values",
    logic: (matcher, emitter) => {
        const cell_value1 = matcher.pool.getOne(cell_value);
        const cell_value2 = matcher.pool.getOne(cell_value);

        matcher.where(obj.op.cmp(cell_value1.cell, "==", cell_value2.cell));
        matcher.where(int.op.cmp(cell_value1.value, "!=", cell_value2.value));

        emitter.emitError();
    },
});

const no_zero_allowed_values = classic_mod.step({
    name: "no_zero_allowed_values",
    logic: (matcher, emitter) => {
        const allowed_values1 = matcher.pool.getOne(allowed_values);

        matcher.where(obj.op.cmp(set.op.size(allowed_values1.values), "==", 0));

        emitter.emitError();
    },
});

const no_zero_required_value_cells = classic_mod.step({
    name: "no_zero_required_value_cells",
    logic: (matcher, emitter) => {
        const required_value1 = matcher.pool.getOne(required_value);

        matcher.where(obj.op.cmp(set.op.size(required_value1.cells), "==", 0));

        emitter.emitError();
    },
});

const no_insufficient_split = classic_mod.step({
    name: "no_insufficient_split",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(non_repeat_set);

        const allowed_values_set = matcher.pool.getMany(allowed_values);
        matcher.where(
            set.op.cmp(
                set.op.map(allowed_values_set, (x) => {
                    return x.cell;
                }),
                "subset of",
                set1.cells,
            ),
        );
        const values = set.op.union(
            set.op.map(allowed_values_set, (x) => {
                return x.values;
            }),
        );
        matcher.require(
            int.op.cmp(
                set.op.size(values),
                "<",
                set.op.size(allowed_values_set),
            ),
        );

        emitter.emitError();
    },
});

const no_repeat_in_non_repeat = classic_mod.step({
    name: "no_repeat_in_non_repeat",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(non_repeat_set);

        const required_value1 = matcher.pool.getOne(required_value);
        const required_value2 = matcher.pool.getOne(required_value);

        matcher.where(
            set.op.cmp(required_value1.cells, "subset of", set1.cells),
        );
        matcher.where(
            set.op.cmp(required_value2.cells, "subset of", set1.cells),
        );
        matcher.where(
            int.op.cmp(required_value1.value, "==", required_value2.value),
        );
        matcher.require(
            int.op.cmp(
                set.op.size(
                    set.op.intersect([
                        required_value1.cells,
                        required_value2.cells,
                    ]),
                ),
                "==",
                0,
            ),
        );

        emitter.emitError();
    },
});

const all_values_present = classic_mod.step({
    name: "all_values_present",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(full_set);

        const allowed_values_set = matcher.pool.getMany(allowed_values);

        matcher.where(
            obj.op.cmp(
                set.op.map(allowed_values_set, (x) => {
                    return x.cell;
                }),
                "==",
                set1.cells,
            ),
        );
        matcher.require(
            set.op.cmp(
                set1.values,
                "subset of",
                set.op.union(
                    set.op.map(allowed_values_set, (x) => {
                        return x.values;
                    }),
                ),
            ),
        );

        emitter.emitError();
    },
});
