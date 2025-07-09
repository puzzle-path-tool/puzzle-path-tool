import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    full_set,
    non_repeat_set,
    required_value,
} from "./deductions";

declare const quantor: any;
declare const set: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

const required_from_full_set = classic_mod.step({
    name: "required_from_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set);

        const cell_set = matcher.pool.get_many(allowed_values.cells);
        matcher.where(set.do(cell_set, "true subset of", set1.cells));
        const value = matcher.get_one(int);
        matcher.where(
            quantor.all((matcher: TODO) => {
                const allowed_values1 = matcher.pool.get_one(allowed_values);
                matcher.require(
                    set.do(allowed_values1.cell, "element of", cell_set),
                );
                matcher.require(
                    set.do(allowed_values1.values, "contains", value),
                );
            }),
        );
        matcher.require(
            quantor.all((matcher: TODO) => {
                const allowed_values1 = matcher.pool.get_one(allowed_values);
                matcher.where(
                    set.do(
                        allowed_values1.cell,
                        "element of",
                        set.do(set1.cells, "without", cell_set),
                    ),
                );
                matcher.require(
                    set.do(allowed_values1.values, "contains not", value),
                ); //todo
            }),
        );

        emitter.emit(required_value, [
            {
                cells: cell_set,
                value: value,
            },
        ]);
    },
});

const required_in_non_repeat_set = classic_mod.step({
    name: "required_in_non_repeat_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(non_repeat_set);
        const required_value1 = matcher.pool.get_one(required_value);
        matcher.require(
            set.do(required_value1.cells, "true subset of", set1.cells),
        );

        const allowed_values_set = matcher.pool.get_many(allowed_values, {
            invalidate: true,
        });
        matcher.where(
            set.do(
                set.op.map(allowed_values_set, (x: TODO) => {
                    return x.cell;
                }),
                "==",
                required_value1.cells,
            ),
        );

        emitter.emit(
            allowed_values,
            set.op.map(allowed_values_set, (x: TODO) => {
                return {
                    cell: x.cell,
                    values: set.do(x.values, "without", required_value1.value),
                };
            }),
        );
    },
});

const required_set_to_allowed = classic_mod.step({
    name: "required_set_to_allowed",
    logic: (matcher: TODO, emitter: TODO) => {
        const required_value_set = matcher.pool.get_many(required_value);
        const cells = set.union(
            set.op.map(required_value_set, (x: TODO) => {
                return x.cells;
            }),
        );
        matcher.require(
            int.op.cmp(required_value_set.size(), "==", cells.size),
        );

        emitter.emit(
            allowed_values,
            set.op.map(cells, (i: TODO) => {
                return {
                    cell: i,
                    values: set.op.map(required_value_set, (ii: TODO) => {
                        return ii.value;
                    }),
                };
            }),
        );
    },
});
