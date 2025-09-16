import { int, obj, quantor, set } from "api/prelude";
import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    full_set,
    matching_cells,
    non_repeat_set,
} from "./deductions";

const matching_from_full_set = classic_mod.step({
    name: "matching_from_full_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(full_set);
        const set2 = matcher.pool.getOne(full_set);
        matcher.require(obj.op.cmp(set1, "!=", set2));
        matcher.require(obj.op.cmp(set1.values, "!=", set2.values));

        const set1w2 = set.op.join(set1.cells, "without", set2.cells);
        const set2w1 = set.op.join(set2.cells, "without", set1.cells);

        matcher.where(int.op.cmp(set.op.size(set1w2), "==", 1));
        matcher.where(int.op.cmp(set.op.size(set2w1), "==", 1));

        emitter.emitOne(matching_cells, {
            cells: set.op.join(set1w2, "union", set2w1),
        });
    },
});

const increase_matching_from_full_set = classic_mod.step({
    name: "increase_matching_from_full_set",
    logic: (matcher, emitter) => {
        const set1 = matcher.pool.getOne(full_set);
        const matching_cells1 = matcher.pool.getOne(matching_cells, {
            invalidate: true,
        });
        const allowed_values1 = matcher.pool.getOne(allowed_values);

        matcher.where(
            set.op.cmp(allowed_values1.values, "subset of", set1.values),
        );
        matcher.where(
            set.op.element_of(
                allowed_values1.cell,
                "element of",
                matching_cells1.cells,
            ),
        );
        matcher.where(
            int.op.cmp(
                set.op.size(
                    set.op.intersect(set1.cells, matching_cells1.cells),
                ),
                "==",
                0,
            ),
        );

        const non_repeat_set_set = matcher.pool.getMany(non_repeat_set);
        matcher.where(
            quantor.op.all((matcher) => {
                const current_non_repeat_set =
                    matcher.pool.getOne(non_repeat_set);
                set.op.element_of(
                    current_non_repeat_set,
                    "element of",
                    non_repeat_set_set,
                );
                matcher.require(
                    int.op.cmp(
                        set.op.size(
                            set.op.intersect(
                                current_non_repeat_set.cells,
                                matching_cells1.cells,
                            ),
                        ),
                        "==",
                        1,
                    ),
                );
            }),
        );

        const cells1 = set.op.join(
            set1.cells,
            "without",
            set.op.fold(
                "union",
                set.op.map(non_repeat_set_set, (x) => {
                    return x.cells;
                }),
            ),
        );
        matcher.require(int.op.cmp(set.op.size(cells1), "==", 1));

        emitter.emitOne(matching_cells, {
            cells: set.op.union(matching_cells1.cells, cells1),
        });
    },
});
