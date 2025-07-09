import { classic_mod } from "../classic_mod";
import { allowed_values, full_set, non_repeat_set } from "./deductions";

declare const quantor: any;
declare const set: any;
declare const int: any;
declare const obj: any;
type TODO = any;
const todo = "TODO";

const split_full_set_step = classic_mod.step({
    name: "split_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set);

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
        matcher.require(int.op.cmp(values.size, "==", allowed_values_set.size));

        const set2 = {
            cells: set.op.map(allowed_values_set, (x: TODO) => {
                return x.cell;
            }),
            values: values,
        };

        emitter.emit(full_set, [
            set2,
            {
                cells: set.do(set1.cells, "without", set2.cells),
                values: set.do(set1.values, "without", set2.values),
            },
        ]);

        const outer_allowed_values_set = matcher.pool.get_many(
            allowed_values_set,
            {
                invalidate: true,
            },
        );
        matcher.require(
            obj.op.cmp(
                set.do(set1.cells, "without", set2.cells),
                "==",
                set.op.map(outer_allowed_values_set, (x: TODO) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emit(
            allowed_values,
            set.op.map(outer_allowed_values_set, (i: TODO) => {
                return {
                    values: set.op.map(i.values, (ii: TODO) => {
                        return set.do(ii, "without", set2.values);
                    }),
                    cell: i.cell,
                };
            }),
        );
    },
});

const split_non_repeat_set_step = classic_mod.step({
    name: "split_non_repeat_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(non_repeat_set);

        const allowed_values_set = matcher.pool.get_many(allowed_values);
        matcher.require(
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
        matcher.require(int.op.cmp(values.size, "==", allowed_values_set.size));

        const set2 = {
            cells: set.op.map(allowed_values_set, (x: TODO) => {
                return x.cell;
            }),
            values: values,
        };

        emitter.emit(full_set, [set2]);

        const outer_allowed_values_set = matcher.pool.get_many(allowed_values, {
            invalidate: true,
        });
        matcher.where(
            obj.op.cmp(
                set.do(set1.cells, "without", set2.cells),
                "==",
                set.op.map(outer_allowed_values_set, (x: TODO) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emit(
            allowed_values,
            set.op.map(outer_allowed_values_set, (i: TODO) => {
                return {
                    values: set.op.map(i.values, (ii: TODO) => {
                        return set.do(ii, "without", set2.values);
                    }),
                    cell: i.cell,
                };
            }),
        );
    },
});
