import { int, obj, set } from "api/api";
import { classic_mod } from "../classic_mod";
import { position } from "../utils/position";

export const allowed_values = classic_mod.deduction({
    name: "allowed_values",
    create: () => {
        const allowed_values = obj.decl({
            values: set.decl(int.decl()),
            cell: position.decl(),
        });
        return { decl: allowed_values };
    },
});

export const required_value = classic_mod.deduction({
    name: "required_value",
    create: () => {
        const required_value = obj.decl({
            value: int.decl(),
            cells: set.decl(position.decl()),
        });
        return { decl: required_value };
    },
});

export const matching_cells = classic_mod.deduction({
    name: "matching_cells",
    create: () => {
        const matching_cells = obj.decl({
            cells: set.decl(position.decl()),
        });
        return { decl: matching_cells };
    },
});

export const non_repeat_set = classic_mod.deduction({
    name: "non_repeat_set",
    create: () => {
        const non_repeat_set = obj.decl({
            cells: set.decl(position.decl()),
        });
        return { decl: non_repeat_set };
    },
});

export const full_set = classic_mod.deduction({
    name: "full_set",
    create: () => {
        const full_set = obj.decl({
            values: set.decl(int.decl()),
            cells: set.decl(position.decl()),
        });
        return { decl: full_set };
    },
});

const non_repeat_set_from_full_set = classic_mod.step({
    name: "non_repeat_set_from_full_set",
    logic: (matcher, emitter) => {
        const full_set1 = matcher.pool.getOne(full_set);

        emitter.emitOne(non_repeat_set, { cells: full_set1.cells });
    },
});

const allowed_values_from_full_set = classic_mod.step({
    name: "allowed_values_from_full_set",
    logic: (matcher, emitter) => {
        const full_set1 = matcher.pool.getOne(full_set);

        emitter.emitOneOf(
            allowed_values,
            set.op.map(full_set1.cells, (x) => {
                return { values: full_set1.values, cell: x };
            }),
        );
    },
});

export const cell_value = classic_mod.deduction({
    name: "cell_value",
    create: () => {
        const cell_value = obj.decl({
            value: int.decl(),
            cell: position.decl(),
        });
        return { decl: cell_value };
    },
});

const required_from_cell_value = classic_mod.step({
    name: "cell_value_required",
    logic: (matcher, emitter) => {
        const cell_value1 = matcher.pool.getOne(cell_value);

        emitter.emitOne(required_value, {
            value: cell_value1.value,
            cells: [cell_value1.cell],
        });
    },
});

const allowed_from_cell_value = classic_mod.step({
    name: "cell_value_allowed",
    logic: (matcher, emitter) => {
        const cell_value1 = matcher.pool.getOne(cell_value);

        emitter.emitOne(allowed_values, {
            values: [cell_value1.value],
            cell: cell_value1.cell,
        });
    },
});

const cell_value_from_required = classic_mod.step({
    name: "one_cell_required",
    logic: (matcher, emitter) => {
        const required_value1 = matcher.pool.getOne(required_value);

        matcher.where(int.op.cmp(set.op.size(required_value1.cells), "==", 1));

        const position = matcher.getOne(cell_value.t.cell);
        matcher.require(
            set.op.element_of(position, "element of", required_value1.cells),
        );

        emitter.emitOne(cell_value, {
            value: required_value1.value,
            cell: position,
        });
    },
});

const cell_value_from_allowed = classic_mod.step({
    name: "one_value_allowed",
    logic: (matcher, emitter) => {
        const allowed_values1 = matcher.pool.getOne(allowed_values);

        matcher.where(int.op.cmp(set.op.size(allowed_values1.values), "==", 1));

        emitter.emitOne(cell_value, {
            value: int.op.fold("+", allowed_values1.values),
            cell: allowed_values1.cell,
        });
    },
});

export const square_bounds = classic_mod.deduction({
    name: "bounds",
    create: () => {
        const square_bounds = obj.decl({
            length: int.decl(),
        });
        return { decl: square_bounds };
    },
});
