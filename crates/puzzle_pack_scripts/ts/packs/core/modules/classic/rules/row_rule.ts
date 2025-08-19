import { int, obj, set } from "api/api";
import { classic_mod } from "../../classic_mod";
import { position } from "../../utils/position";
import { full_set } from "../deductions";

export const row_rule = classic_mod.rule({
    name: "row_rule",
    create: () => {
        const row_rule = obj.decl({
            values: set.decl(int.decl()),
            cells: set.decl(position.decl()),
        });
        return { decl: row_rule };
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "row_to_full_set",
    logic: (matcher, emitter) => {
        const row1 = matcher.pool.getOne(row_rule);

        emitter.emitOne(full_set, { cells: row1.cells, values: row1.values });
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher, emitter) => {
        const row = matcher.pool.getOne(row_rule);

        emitter.emitRuleResolved(row_rule, row);
    },
});
