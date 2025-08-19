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

        const cell_set = matcher.pool.getMany(allowed_values.cells);
        matcher.where(set.op.cmp(cell_set, "true subset of", set1.cells));
        const value = matcher.getOne(int);
        matcher.where(
            quantor.all((matcher) => {
                const allowed_values1 = matcher.pool.getOne(allowed_values);
                matcher.require(
                    set.op.element_of(allowed_values1.cell, "element of", cell_set),
                );
                matcher.require(
                    set.op.element_of(allowed_values1.values, "contains", value),
                );
            }),
        );
        matcher.require(
            quantor.all((matcher) => {
                const allowed_values1 = matcher.pool.getOne(allowed_values);
                matcher.where(
                    set.op.element_of(
                        allowed_values1.cell,
                        "element of",
                        set.do(set1.cells, "without", cell_set),
                    ),
                );
                matcher.require(
                    set.op.contains(allowed_values1.values, "contains not", value),
                ); //todo
            }),
        );

        emitter.emitOne(required_value,
            {
                cells: cell_set,
                value: value,
            },
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

        emitter.emitOne(
            allowed_values,
            set.op.map(allowed_values_set, (x) => {
                return {
                    cell: x.cell,
                    values: set.op.disjunctive_union(x.values, "without", required_value1.value),
                };
            }),
        );
    },
});

const required_set_to_allowed = classic_mod.step({
    name: "required_set_to_allowed",
    logic: (matcher, emitter) => {
        const required_value_set = matcher.pool.getMany(required_value);
        const cells = set.op.union(
            set.op.map(required_value_set, (x) => {
                return x.cells;
            }),
        );
        matcher.require(
            int.op.cmp(set.op.size(required_value_set), "==", set.op.size(cells)),
        );

        emitter.emitOne(
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
