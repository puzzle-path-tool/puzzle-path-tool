import { classic_mod } from "../classic_mod";

declare const field: any;
declare const array: any;
declare const int: any;
declare const set: any;
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

export const required_value = classic_mod.deduction({
    name: "required_value",
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

const non_repeat_set_from_full_set = classic_mod.step({
    name: "non_repeat_set_from_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const full_set1 = matcher.pool.get_one(full_set);

        emitter.emit(non_repeat_set, [{ cells: full_set1.cells }]);
    },
});

const allowed_values_from_full_set = classic_mod.step({
    name: "non_repeat_set_from_full_set",
    logic: (matcher: TODO, emitter: TODO) => {
        const full_set1 = matcher.pool.get_one(full_set);

        emitter.emit(
            non_repeat_set,
            set.op.map(full_set1.cells, (x: TODO) => {
                return { values: full_set1.values, cell: x };
            }),
        );
    },
});

export const cell_value = classic_mod.deduction({
    name: "cell_value",
    data: {
        value: int.field,
        cell: position(),
    },
});

const required_from_cell_value = classic_mod.step({
    name: "cell_value_required",
    logic: (matcher: TODO, emitter: TODO) => {
        const cell_value1 = matcher.pool.get_one(cell_value);

        emitter.emit(required_value, [
            {
                value: cell_value1.value,
                cells: [cell_value1.cell],
            },
        ]);
    },
});

const allowed_from_cell_value = classic_mod.step({
    name: "cell_value_allowed",
    logic: (matcher: TODO, emitter: TODO) => {
        const cell_value1 = matcher.pool.get_one(cell_value);

        emitter.emit(allowed_values, [
            {
                values: [cell_value1.value],
                cell: cell_value1.cell,
            },
        ]);
    },
});

const cell_value_from_required = classic_mod.step({
    name: "one_cell_required",
    logic: (matcher: TODO, emitter: TODO) => {
        const required_value1 = matcher.pool.get_one(required_value);

        matcher.where(int.op.cmp(int.op.size(required_value1.cell), "==", 1));

        const position = matcher.get_one(cell_value.cell);
        matcher.require(set.do(position, "element of", required_value1.cells));

        emitter.emit(required_value, [
            {
                value: required_value1.value,
                cell: position,
            },
        ]);
    },
});

const cell_value_from_allowed = classic_mod.step({
    name: "one_value_allowed",
    logic: (matcher: TODO, emitter: TODO) => {
        const allowed_values1 = matcher.pool.get_one(allowed_values);

        matcher.where(int.op.cmp(int.op.size(allowed_values1.values), "==", 1));

        emitter.emit(allowed_values, [
            {
                value: int.op.sum(allowed_values1.values),
                cell: allowed_values1.cell,
            },
        ]);
    },
});

export const square_bounds = classic_mod.deduction({
    name: "bounds",
    data: {
        length: int.field,
    },
});
