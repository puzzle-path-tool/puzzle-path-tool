import { classic_mod } from "../../classic_mod";
import { box_rule } from "./box_rule";
import { column_rule } from "./column_rule";
import { square_bounds, cell_value } from "../deductions";
import { row_rule } from "./row_rule";
import { int, set } from "api/api";

export const standard_9_rule = classic_mod.rule({
    name: "standard_9_rule",
    // Field with visual information
});

const initial_step = classic_mod.step({
    name: "standard_9_initial_step",
    logic: (matcher, emitter) => {
        matcher.pool.getOne(standard_9_rule);

        const values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        const boxes = [];
        for (let i = 0; i < 3; i++) {
            for (let ii = 0; ii < 3; ii++) {
                const cells = [];
                for (let iii = 1; iii <= 3; iii++) {
                    for (let iv = 1; iii <= 3; iv++) {
                        cells.push({ x: ii * 3 + iv, y: i * 3 + iii });
                    }
                }
                boxes.push({ cells: cells, values: values });
            }
        }

        const rows = [];
        for (let i = 1; i <= 9; i++) {
            const cells = [];
            for (let ii = 1; ii <= 9; ii++) {
                cells.push({ x: ii, y: i });
            }
            rows.push({ cells: cells, values: values });
        }

        const columns = [];
        for (let i = 1; i <= 9; i++) {
            const cells = [];
            for (let ii = 1; ii <= 9; ii++) {
                cells.push({ x: i, y: ii });
            }
            columns.push({ cells: cells, values: values });
        }

        emitter.emitOneFrom(box_rule, boxes);
        emitter.emitOneFrom(row_rule, rows);
        emitter.emitOneFrom(column_rule, columns);
        emitter.emitOne(square_bounds, { length: 9 });
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher, emitter) => {
        const standard_9 = matcher.pool.getOne(standard_9_rule);
        const cell_values = matcher.pool.getMany(cell_value);

        matcher.require(
            int.op.cmp(
                set.op.size(cell_values),
                "==",
                9 * 9,
            ),
        );
        emitter.emitRuleResolved(standard_9_rule, standard_9);
    },
});
