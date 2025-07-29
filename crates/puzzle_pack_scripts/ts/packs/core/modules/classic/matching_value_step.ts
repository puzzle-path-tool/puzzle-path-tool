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
        matcher.require(set.op.cmp(set1, "!=", set2));

        const set1w2 = set.do(set1.cells, "without", set2.cells);
        const set2w1 = set.do(set2.cells, "without", set1.cells);

        matcher.where(int.op.cmp(set1w2.size, "==", 1));
        matcher.where(int.op.cmp(set2w1.size, "==", 1));

        const cell1 = matcher.pool.getOne(allowed_values.cell);
        matcher.require(set.do(cell1, "element of", set1w2));

        const cell2 = matcher.pool.getOne(allowed_values.cell);
        matcher.require(set.do(cell1, "element of", set2w1));

        emitter.emitOne(matching_cells, [
            {
                cells: [cell1, cell2],
            },
        ]);
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

        matcher.where(set.do(allowed_values1.values, "subset of", set1.values));
        matcher.where(
            set.do(allowed_values1.cell, "element of", matching_cells1.cells),
        );
        matcher.where(
            int.op.cmp(
                set.do(set1.cells, "intersection", matching_cells1.cells).size,
                "==",
                0,
            ),
        );

        const non_repeat_set_set = matcher.pool.getMany(non_repeat_set);
        matcher.where(
            quantor.all((matcher) => {
                const current_non_repeat_set =
                    matcher.pool.getOne(non_repeat_set);
                matcher.require(
                    int.op.cmp(
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
                    set.op.map(non_repeat_set_set, (x: TODO) => {
                        return x.cells;
                    }),
                ),
            ),
        );
        matcher.require(int.op.cmp(cells1.size, "==", 1));

        emitter.emitOne(matching_cells, [
            { cells: set.do(matching_cells1.cells, "union", cells1) },
        ]);
    },
});
