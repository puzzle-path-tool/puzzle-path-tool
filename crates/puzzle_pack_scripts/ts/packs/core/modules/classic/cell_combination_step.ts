import { bool, int, obj, set } from "api/api";
import { classic_mod } from "../classic_mod";
import { allowed_values, matching_cells, required_value } from "./deductions";

const combine_required_allowed = classic_mod.step({
    name: "combine_required_allowed",
    logic: (matcher, emitter) => {
        const required_value1 = matcher.pool.getOne(required_value, {
            invalidate: true,
        });
        const allowed_values1 = matcher.pool.getOne(allowed_values);

        matcher.where(
            set.op.element_of(
                allowed_values1.cell,
                "element of",
                required_value1.cells,
            ),
        );

        matcher.where(
            bool.op.not(
                set.op.element_of(
                    required_value1.value,
                    "element of",
                    allowed_values1.values,
                ),
            ),
        );

        emitter.emitOne(required_value, {
            value: required_value1.value,
            cells: set.op.disjunctive_union(required_value1.cells, [
                allowed_values1.cell,
            ]),
        });
    },
});

const combine_allowed = classic_mod.step({
    name: "combine_allowed",
    logic: (matcher, emitter) => {
        const allowed_values1 = matcher.pool.getOne(allowed_values, {
            invalidate: true,
        });
        const allowed_values2 = matcher.pool.getOne(allowed_values, {
            invalidate: true,
        });

        matcher.where(obj.op.cmp(allowed_values1, "!=", allowed_values2));

        matcher.where(
            obj.op.cmp(allowed_values1.cell, "==", allowed_values2.cell),
        );

        emitter.emitOneFrom(allowed_values, [
            {
                values: set.op.intersect(
                    allowed_values1.values,
                    allowed_values2.values,
                ),
                cell: allowed_values1.cell,
            },
        ]);
    },
});

const combine_allowed_matching = classic_mod.step({
    name: "combine_allowed_matching",
    logic: (matcher, emitter) => {
        const allowed_values_set = matcher.pool.getMany(allowed_values, {
            invalidate: true,
        });
        const matching_cells1 = matcher.pool.getOne(matching_cells);

        matcher.where(
            obj.op.cmp(
                set.op.map(allowed_values_set, (x) => x.cell),
                "==",
                matching_cells1.cells,
            ),
        );
        const values_set = set.op.map(allowed_values_set, (x) => x.values);
        const values_intersect = set.op.fold("intersect", values_set);
        matcher.require(
            set.op.cmp(set.op.fold("union", values_set), "!=", values_intersect),
        );

        emitter.emitOneFrom(
            allowed_values,
            set.op.map(allowed_values_set, (x) => {
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
    logic: (matcher, emitter) => {
        const matching_cells1 = matcher.pool.getOne(matching_cells, {
            invalidate: true,
        });
        const matching_cells2 = matcher.pool.getOne(matching_cells, {
            invalidate: true,
        });

        matcher.where(obj.op.cmp(matching_cells1, "!=", matching_cells2));

        matcher.where(
            int.op.cmp(
                set.op.size(
                    set.op.intersect([matching_cells1, matching_cells2]),
                ),
                ">=",
                1,
            ),
        );
        emitter.emitOne(matching_cells, {
            cells: set.op.union(matching_cells1.cells, matching_cells2.cells),
        });
    },
});
