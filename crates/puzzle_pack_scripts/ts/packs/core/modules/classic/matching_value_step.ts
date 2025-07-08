import { classic_mod } from "../classic_mod";
import {
    allowed_values,
    full_set,
    matching_cells,
    non_repeat_set,
} from "./deductions";

declare const quantor: any;
declare const set: any;
declare const cmp: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

const matching_from_full_set = classic_mod.step({
    name: "matching_from_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set);
        const set2 = matcher.pool.get_one(full_set);
        const set1w2 = set.do(set1.cells, "without", set2.cells);
        const set2w1 = set.do(set2.cells, "without", set1.cells);

        matcher.where(cmp.do(set1w2.size, "==", 1));
        matcher.where(cmp.do(set2w1.size, "==", 1));

        const cell1 = matcher.get_one(allowed_values.cell);
        matcher.require(set.do(cell1, "element of", set1w2));

        const cell2 = matcher.get_one(allowed_values.cell);
        matcher.require(set.do(cell1, "element of", set2w1));

        emitter.emit(matching_cells, [
            {
                cells: [cell1, cell2],
            },
        ]);
    },
});

const increase_matching_from_full_set = classic_mod.step({
    name: "increase_matching_from_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set);
        const matching_cells1 = matcher.pool.get_one(matching_cells, {
            invalidate: true,
        });
        const allowed_values1 = matcher.pool.get_one(allowed_values);

        matcher.where(set.do(allowed_values1.values, "subset of", set1.values));
        matcher.where(
            set.do(allowed_values1.cell, "element of", matching_cells1.cells),
        );
        matcher.where(
            cmp.do(
                set.do(set1.cells, "intersection", matching_cells1.cells).size,
                "==",
                0,
            ),
        );

        const non_repeat_set_set = matcher.pool.get_many(non_repeat_set);
        matcher.where(
            quantor.all((matcher: TODO) => {
                const current_non_repeat_set =
                    matcher.pool.get_one(non_repeat_set);
                matcher.require(
                    cmp.do(
                        set.do(
                            current_non_repeat_set.cells,
                            "intersection",
                            matching_cells1.cells,
                        ).size,
                        "==",
                        1,
                    ),
                );
            }),
        );

        const cells1 = set.do(
            set1.cells,
            "without",
            set.do(
                set.union(
                    non_repeat_set_set.map((x: TODO) => {
                        return x.cells;
                    }),
                ),
            ),
        );
        matcher.require(cmp.do(cells1.size, "==", 1));

        emitter.emit(matching_cells, [
            { cells: set.do(matching_cells1.cells, "union", cells1) },
        ]);
    },
});
