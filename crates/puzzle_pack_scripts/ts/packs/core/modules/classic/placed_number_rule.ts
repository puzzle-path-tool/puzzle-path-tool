import { int, obj } from "api/prelude";
import { classic_mod } from "../classic_mod";
import { position } from "../utils/position";
import { cell_value } from "./deductions";

export const placed_number_rule = classic_mod.rule({
    name: "placed_number_rule",
    create: () => {
        const placed_number_rule = obj.decl({
            cell: position.decl(),
            value: int.decl(),
        });
        return { decl: placed_number_rule };
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "placed_number_to_cell_value",
    logic: (matcher, emitter) => {
        const placed_number1 = matcher.pool.getOne(placed_number_rule);

        emitter.emitOne(cell_value,
            { cell: placed_number1.cell, value: placed_number1.value },
        );
    },
});
