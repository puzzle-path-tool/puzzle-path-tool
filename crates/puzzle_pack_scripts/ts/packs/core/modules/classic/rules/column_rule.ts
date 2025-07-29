import { int, obj, set } from "api/api";
import { classic_mod } from "../../classic_mod";
import { position } from "../../utils/position";
import { full_set } from "../deductions";

export const column_rule = classic_mod.rule({
    name: "column_rule",
    create: () => {
        const column_rule = obj.decl({
            cells: set.decl(position.decl()),
            values: set.decl(int.decl()),
        });
        return { decl: column_rule };
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "column_to_full_set",
    logic: (matcher, emitter) => {
        const column1 = matcher.pool.getOne(column_rule);

        emitter.emitOne(full_set, [
            { cells: column1.cells, values: column1.values },
        ]);
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher, emitter) => {
        const column = matcher.pool.getOne(column_rule);

        emitter.emitRuleResolved(column_rule, column);
    },
});
