import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    full_set,
    non_repeat_set,
    required_values,
} from "./deductions";

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
                x.cell;
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

        const outer_allowed_values_set = matcher.get_many(allowed_values_set);
        matcher.require(
            cmp.do(
                set.do(set1.cells, "without", set2.cells),
                "==",
                outer_allowed_values_set.map((x: TODO) => {
                    x.cell;
                }),
            ),
        );
        matcher.require(
            quantor.all((matcher: TODO) => {
                const value1 = matcher.get_one(allowed_values);
                matcher.where(
                    set.do(value1, "element of", outer_allowed_values_set),
                );
                const value2 = matcher.get_one(allowed_values);
                matcher.pool.where(
                    set.do(
                        value2.cell,
                        "element of",
                        outer_allowed_values_set.map((x: TODO) => {
                            x.cell;
                        }),
                    ),
                );

                matcher.require(
                    cmp.do(
                        value1.values,
                        "==",
                        set.do(value2.values, "without", set2.values),
                    ),
                );
            }),
        );

        emitter.emit(allowed_values, outer_allowed_values_set);
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
                x.cell;
            }),
            values: values,
        };

        emitter.emit(full_set, [set2]);

        const outer_allowed_values_set = matcher.get_many(allowed_values_set);
        matcher.require(
            cmp.do(
                set.do(set1.cells, "without", set2.cells),
                "==",
                outer_allowed_values_set.map((x: TODO) => {
                    x.cell;
                }),
            ),
        );
        matcher.require(
            quantor.all((matcher: TODO) => {
                const value1 = matcher.get_one(allowed_values);
                matcher.where(
                    set.do(value1, "element of", outer_allowed_values_set),
                );
                const value2 = matcher.get_one(allowed_values);
                matcher.pool.where(
                    set.do(
                        value2.cell,
                        "element of",
                        outer_allowed_values_set.map((x: TODO) => {
                            x.cell;
                        }),
                    ),
                );

                matcher.require(
                    cmp.do(
                        value1.values,
                        "==",
                        set.do(value2.values, "without", set2.values),
                    ),
                );
            }),
        );

        emitter.emit(allowed_values, outer_allowed_values_set);
    },
});
