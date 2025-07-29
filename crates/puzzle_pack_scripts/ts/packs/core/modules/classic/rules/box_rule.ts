import { int, obj, set } from "api/api";
import { classic_mod } from "../../classic_mod";
import { position } from "../../utils/position";
import { full_set } from "../deductions";
import { column_rule } from "./column_rule";

export const box_rule = classic_mod.rule({
    name: "box_rule",
    description: "Field with visual information",
    create: () => {
        const box_rule = obj.decl({
            cells: set.decl(position.decl()),
            values: set.decl(int.decl()),
        });
        return { decl: box_rule };
    },
});

export const to_full_set = classic_mod.step({
    name: "box_to_full_set",
    logic: (matcher, emitter) => {
        const box1 = matcher.pool.getOne(box_rule);

        emitter.emitOne(full_set, { cells: box1.cells, values: box1.values });
    },
});

const resolve_step = classic_mod.step({
    name: "resolve_standard_9",
    logic: (matcher, emitter) => {
        const box = matcher.pool.getOne(box_rule);

        emitter.emitRuleResolved(box_rule, box);
    },
});
