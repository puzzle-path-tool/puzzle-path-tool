import { classic_mod } from "../classic_mod";
import { allowed_values, matching_cells, required_value } from "./deductions";

declare const quantor: any;
declare const set: any;
declare const cmp: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

const combine_required_allowed = classic_mod.step({
    name: "combine_required_allowed",
    logic: (matcher: TODO, emitter: TODO) => {
        const required_value1 = matcher.pool.get_one(required_value);
        const allowed_values1 = matcher.pool.get_one(allowed_values);

        matcher.where(
            set.do(allowed_values1.cell, "element of", required_value1.cells),
        );

        matcher.where(
            set.do(
                required_value1.value,
                "not element of",
                allowed_values1.values,
            ), //todo
        );

        emitter.emit(required_value, [
            {
                value: required_value1.value,
                cells: set.do(required_value1.cells, "without", [
                    allowed_values1.cell,
                ]),
            },
        ]);
    },
});

const combine_allowed = classic_mod.step({
    name: "combine_allowed",
    logic: (matcher: TODO, emitter: TODO) => {
        const allowed_values1 = matcher.pool.get_one(allowed_values);
        const allowed_values2 = matcher.pool.get_one(allowed_values);

        matcher.where(cmp.do(allowed_values1, "/=", allowed_values2));

        matcher.where(cmp.do(allowed_values1.cell, "==", allowed_values2.cell));

        emitter.emit(allowed_values, [
            {
                values: set.do(
                    allowed_values1.values,
                    "intersection",
                    allowed_values2.values,
                ),
                cell: allowed_values1.cell,
            },
        ]);
    },
});

const combine_allowed_matching = classic_mod.step({
    name: "combine_allowed_matching",
    logic: (matcher: TODO, emitter: TODO) => {
        const allowed_values_set = matcher.pool.get_many(allowed_values);
        const matching_cells1 = matcher.pool.get_one(matching_cells);

        matcher.where(
            cmp.do(
                allowed_values_set.map((x: TODO) => {
                    return x.cell;
                }),
                "==",
                matching_cells1.cells,
            ),
        );
        const values_set = allowed_values_set.map((x: TODO) => {
            return x.values;
        });
        const values_intersect = set.intersect(values_set);
        matcher.require(cmp.do(set.union(values_set), "/=", values_intersect));

        emitter.emit(
            allowed_values,
            allowed_values_set.map((x: TODO) => {
                return {
                    values: values_intersect,
                    cell: x.cell,
                };
            }),
        );
    },
});

const combine_matching = classic_mod.step({
    name: "combine_matching",
    logic: (matcher: TODO, emitter: TODO) => {
        const matching_cells1 = matcher.pool.get_one(matching_cells);
        const matching_cells2 = matcher.pool.get_one(matching_cells);

        matcher.where(cmp.do(matching_cells1, "/=", matching_cells2));

        matcher.where(
            cmp.do(
                set.do(matching_cells1, "intersection", matching_cells2).size(),
                ">=",
                1,
            ),
        );
        emitter.emit(matching_cells, [
            {
                cells: set.do(
                    matching_cells1.cells,
                    "union",
                    matching_cells2.cells,
                ),
            },
        ]);
        emitter.cosume([matching_cells1, matching_cells2]);
    },
});
