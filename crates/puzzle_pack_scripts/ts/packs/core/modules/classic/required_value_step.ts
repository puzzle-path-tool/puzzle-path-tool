import { int, set } from "api/prelude";
import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    full_set,
    non_repeat_set,
    required_value,
} from "./deductions";

const required_from_full_set = classic_mod.step({
    name: "required_from_full_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(full_set);

        const allowed_values_set1 = matcher.pool.getMany(allowed_values);
        const cell_set = set.op.map(allowed_values_set1, (x) => x.cell);
        matcher.where(set.op.cmp(cell_set, "true subset of", set1.cells));
        const shared_values = set.op.fold(
            "intersect",
            set.op.map(allowed_values_set1, (x) => x.values),
        );

        const allowed_values_set2 = matcher.pool.getMany(allowed_values);
        matcher.where(
            set.op.cmp(
                set.op.map(allowed_values_set2, (x) => x.cell),
                "==",
                set.op.join(set1.cells, "without", cell_set),
            ),
        );
        const values = set.op.join(
            shared_values,
            "without",
            set.op.fold(
                "union",
                set.op.map(allowed_values_set2, (x) => x.values),
            ),
        );

        emitter.emitOneFrom(
            required_value,
            set.op.map(values, (value) => {
                return {
                    cells: cell_set,
                    value: value,
                };
            }),
        );
    },
});

const required_in_non_repeat_set = classic_mod.step({
    name: "required_in_non_repeat_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(non_repeat_set);
        const required_value1 = matcher.pool.getOne(required_value);
        matcher.require(
            set.op.cmp(required_value1.cells, "true subset of", set1.cells),
        );

        const allowed_values_set = matcher.pool.getMany(allowed_values, {
            invalidate: true,
        });
        matcher.where(
            set.op.cmp(
                set.op.map(allowed_values_set, (x) => {
                    return x.cell;
                }),
                "==",
                required_value1.cells,
            ),
        );

        emitter.emitOneFrom(
            allowed_values,
            set.op.map(allowed_values_set, (x) => {
                return {
                    cell: x.cell,
                    values: set.op.join(x.values, "without", [
                        required_value1.value,
                    ]),
                };
            }),
        );
    },
});

const required_set_to_allowed = classic_mod.step({
    name: "required_set_to_allowed",
    logic: (matcher, emitter) => {
        const required_value_set = matcher.pool.getMany(required_value);
        matcher.where(
            int.op.cmp(
                set.op.size(required_value_set),
                "==",
                set.op.size(
                    set.op.union(
                        set.op.map(required_value_set, (x) => {
                            return x.value;
                        }),
                    ),
                ),
            ),
        );

        const cells = set.op.fold(
            "union",
            set.op.map(required_value_set, (x) => {
                return x.cells;
            }),
        );
        matcher.require(
            int.op.cmp(
                set.op.size(required_value_set),
                "==",
                set.op.size(cells),
            ),
        );

        emitter.emitOneFrom(
            allowed_values,
            set.op.map(cells, (i) => {
                return {
                    cell: i,
                    values: set.op.map(required_value_set, (ii) => {
                        return ii.value;
                    }),
                };
            }),
        );
    },
});
