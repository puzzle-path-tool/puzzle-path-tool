import { int, obj, set } from "api/api";
import { classic_mod } from "../classic_mod";
import { allowed_values, full_set, non_repeat_set } from "./deductions";

const split_full_set_step = classic_mod.step({
    name: "split_full_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(full_set);

        const allowed_values_set = matcher.pool.getMany(allowed_values);
        matcher.where(
            set.op.cmp(
                set.op.map(allowed_values_set, (x) => {
                    return x.cell;
                }),
                "true subset of",
                set1.cells,
            ),
        );
        const values = set.op.fold("union",
            set.op.map(allowed_values_set, (x) => x.values),
        );
        matcher.require(int.op.cmp(set.op.size(values), "==", set.op.size(allowed_values_set)));

        const set2 = {
            cells: set.op.map(allowed_values_set, (x) => {
                return x.cell;
            }),
            values: values,
        };

        emitter.emitOne(full_set,
            set2,
        );

        emitter.emitOne(full_set,
            {
                cells: set.op.join(set1.cells, "without", set2.cells),
                values: set.op.join(set1.values, "without", set2.values),
            },
        );

        const outer_allowed_values_set = matcher.pool.getMany(
            allowed_values,
            {
                invalidate: true,
            },
        );
        matcher.require(
            set.op.cmp(
                set.op.join(set1.cells, "without", set2.cells),
                "==",
                set.op.map(outer_allowed_values_set, (x) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emitOneFrom(
            allowed_values,
            set.op.map(outer_allowed_values_set, (i) => {
                return {
                    values: set.op.join(i.values, "without", set2.values),
                    cell: i.cell,
                };
            }),
        );
    },
});

const split_non_repeat_set_step = classic_mod.step({
    name: "split_non_repeat_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(non_repeat_set);

        const allowed_values_set = matcher.pool.getMany(allowed_values);
        matcher.require(
            set.op.cmp(
                set.op.map(allowed_values_set, (x) => {
                    return x.cell;
                }),
                "true subset of",
                set1.cells,
            ),
        );
        const values = set.op.fold("union",
            set.op.map(allowed_values_set, (x) => {
                return x.values;
            }),
        );
        matcher.require(int.op.cmp(set.op.size(values), "==", set.op.size(allowed_values_set)));

        const set2 = {
            cells: set.op.map(allowed_values_set, (x) => {
                return x.cell;
            }),
            values: values,
        };

        emitter.emitOne(full_set, set2);

        const outer_allowed_values_set = matcher.pool.getMany(allowed_values, {
            invalidate: true,
        });
        matcher.where(
            obj.op.cmp(
                set.op.join(set1.cells, "without", set2.cells),
                "==",
                set.op.map(outer_allowed_values_set, (x) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emitOneFrom(
            allowed_values,
            set.op.map(outer_allowed_values_set, (i) => {
                return {
                    values: set.op.join(i.values, "without", set2.values),
                    cell: i.cell,
                };
            }),
        );
    },
});
