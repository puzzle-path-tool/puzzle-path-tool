import { classic_mod } from "../classic_mod";
import { position } from "../test_mod";
import { cell_value } from "./deductions";

declare const quantor: any;
declare const set: any;
declare const int: any;
declare const obj: any;
declare const array: any;
type TODO = any;
const todo = "TODO";

export const placed_number_rule = classic_mod.rule({
    name: "placed_number_rule",
    data: {
        cell: position(),
        value: int.field,
    },
    // Field with visual information
});

const to_full_set = classic_mod.step({
    name: "placed_number_to_cell_value",
    logic: (matcher: TODO, emitter: TODO) => {
        const placed_number1 = matcher.pool.get_one(placed_number_rule);

        emitter.emit(cell_value, [
            { cell: placed_number1.cell, value: placed_number1.value },
        ]);
    },
});
