import { classic_mod } from "../classic_mod";

declare const field: any;
declare const array: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

function position() {
    return field.decl.obj.wrap({
        x: int.field,
        y: int.field,
    });
}

export const allowed_values = classic_mod.deduction({
    name: "allowed_values",
    data: {
        values: array.field(int.field),
        cell: position(),
    },
});

export const required_values = classic_mod.deduction({
    name: "required_values",
    data: {
        value: int.field,
        cells: array.field(position()),
    },
});

export const matching_cells = classic_mod.deduction({
    name: "matching_cells",
    data: {
        cells: array.field(position()),
    },
});

export const non_repeat_set = classic_mod.deduction({
    name: "non_repeat_set",
    data: {
        cells: array.field(position()),
    },
});

export const full_set = classic_mod.deduction({
    name: "full_set",
    data: {
        values: array.field(int.field),
        cells: array.field(position()),
    },
});

export const error = classic_mod.deduction({
    name: "error",
    data: {},
});
