import { classic_mod } from "../classic_mod";
import { allowed_values, full_set, non_repeat_set } from "./deductions";

declare const quantor: any;
declare const set: any;
declare const cmp: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

const split_full_set_step = classic_mod.step({
    name: "split_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set);

        const allowed_values_set = matcher.pool.get_many(allowed_values);
        const values = matcher.get_many(int);
        matcher.where(
            quantor.all((matcher: TODO) => {
                const value = matcher.get_one(allowed_values);
                matcher.where(set.do(value, "element of", allowed_values_set));

                matcher.require(set.do(value.cell, "element of", set1.cells));
                matcher.require(set.do(value.values, "subset of", values));
            }),
        );
        matcher.require(cmp.do(values.size, "==", allowed_values_set.size));

        const set2 = {
            cells: allowed_values_set.map((x: TODO) => {
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
            cmp.do(
                set.do(set1.cells, "without", set2.cells),
                "==",
                outer_allowed_values_set.map((x: TODO) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emit(
            allowed_values,
            outer_allowed_values_set.map((i: TODO) => {
                return {
                    values: i.values.map((ii: TODO) => {
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
        const values = matcher.get_many(int);
        matcher.where(
            quantor.all((matcher: TODO) => {
                const value = matcher.get_one(allowed_values);
                matcher.where(set.do(value, "element of", allowed_values_set));

                matcher.require(set.do(value.cell, "element of", set1.cells));
                matcher.require(set.do(value.values, "subset of", values));
            }),
        );
        matcher.require(cmp.do(values.size, "==", allowed_values_set.size));

        const set2 = {
            cells: allowed_values_set.map((x: TODO) => {
                return x.cell;
            }),
            values: values,
        };

        emitter.emit(full_set, [set2]);

        const outer_allowed_values_set = matcher.pool.get_many(allowed_values, {
            invalidate: true,
        });
        matcher.where(
            cmp.do(
                set.do(set1.cells, "without", set2.cells),
                "==",
                outer_allowed_values_set.map((x: TODO) => {
                    return x.cell;
                }),
            ),
        );

        emitter.emit(
            allowed_values,
            outer_allowed_values_set.map((i: TODO) => {
                return {
                    values: i.values.map((ii: TODO) => {
                        return set.do(ii, "without", set2.values);
                    }),
                    cell: i.cell,
                };
            }),
        );
    },
});
