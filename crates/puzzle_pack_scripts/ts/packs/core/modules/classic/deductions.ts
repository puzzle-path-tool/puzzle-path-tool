import {
    type ObjectField,
    OBJECT_MARKER,
    ARRAY_MARKER,
    INT_MARKER,
    Deduction,
} from "packs/api/api";
import { cell_type, cells_type, values_type } from "../classic_mod";

type AllowedValue = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        values: typeof values_type,
        cell: typeof cell_type,
    },
};

const allowed_values_obj: AllowedValue = {
    __type_marker: OBJECT_MARKER,
    fields: {
        values: values_type,
        cell: cell_type,
    },
};

export const allowed_values: Deduction<AllowedValue> = new Deduction(allowed_values_obj)

type RequiredValue = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        value: { __type_marker: typeof INT_MARKER },
        cells: typeof cells_type,
    },
};

const required_values_obj: RequiredValue = {
    __type_marker: OBJECT_MARKER,
    fields: {
        value: { __type_marker: INT_MARKER },
        cells: cells_type,
    },
};

export const required_values: Deduction<RequiredValue> = new Deduction(required_values_obj)

type MatchingCells = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        cells: typeof cells_type,
    },
};

const matching_cells_obj: MatchingCells = {
    __type_marker: OBJECT_MARKER,
    fields: {
        cells: cells_type,
    },
};

export const matching_cells: Deduction<MatchingCells> = new Deduction(matching_cells_obj)

type NonRepeatSet = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        cells: typeof cells_type,
    },
};

const non_repeat_set_obj: NonRepeatSet = {
    __type_marker: OBJECT_MARKER,
    fields: {
        cells: cells_type,
    },
};

export const non_repeat_set: Deduction<NonRepeatSet> = new Deduction(non_repeat_set_obj)

type FullSet = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        values: typeof values_type,
        cells: typeof cells_type,
    },
};

const full_set_obj: FullSet = {
    __type_marker: OBJECT_MARKER,
    fields: {
        values: values_type,
        cells: cells_type,
    },
};

export const full_set: Deduction<FullSet> = new Deduction(full_set_obj)
