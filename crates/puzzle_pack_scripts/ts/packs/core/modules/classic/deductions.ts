import { classic_mod } from "../classic_mod";

declare const field: any;
type TODO = any;
const todo = "TODO";

function position() {
    return field.decl.obj.wrap(
        {
            x: field.decl.int.wrap({}),
            y: field.decl.int.wrap({}),
        },
    );
}

export const allowed_values = classic_mod.deduction({
    name: "allowed_values",
    data: {
        values: field.decl.arr.wrap(field.decl.int.wrap({})),
        cell: position(),
    },
})

export const required_values = classic_mod.deduction({
    name: "required_values",
    data: {
        value: field.decl.int.wrap({}),
        cells: field.decl.arr.wrap(position()),
    },
})

export const matching_cells = classic_mod.deduction({
    name: "matching_cells",
    data: {
        cells: field.decl.arr.wrap(position()),
    },
})

export const non_repeat_set = classic_mod.deduction({
    name: "non_repeat_set",
    data: {
        cells: field.decl.arr.wrap(position()),
    },
})

export const full_set = classic_mod.deduction({
    name: "full_set",
    data: {
        values: field.decl.arr.wrap(field.decl.int.wrap({})),
        cells: field.decl.arr.wrap(position()),
    },
})

export const error = classic_mod.deduction({
    name: "error",
    data: { },
})
