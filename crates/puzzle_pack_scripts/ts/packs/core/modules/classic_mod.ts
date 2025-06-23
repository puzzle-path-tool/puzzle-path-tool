import {
    ARRAY_MARKER,
    INT_MARKER,
    OBJECT_MARKER,
    type ArrayField,
    type ObjectField,
} from "packs/api/api";
import { core_pack } from "../core_pack";

export const classic_mod = core_pack.module({
    name: "classic",
});

type Cell = {
    __type_marker: typeof OBJECT_MARKER,
    fields: {
        x: { __type_marker: typeof INT_MARKER },
        y: { __type_marker: typeof INT_MARKER },
    },
}

export const cell_type: Cell = {
    __type_marker: OBJECT_MARKER,
    fields: {
        x: { __type_marker: INT_MARKER },
        y: { __type_marker: INT_MARKER },
    },
};

const _cell_test: ObjectField = cell_type

type Cells = {
    __type_marker: typeof ARRAY_MARKER,
    item_type: Cell
}

export const cells_type: Cells = {
    __type_marker: ARRAY_MARKER,
    item_type: cell_type,
};

const _cells_test: ArrayField = cells_type

type Values = {
    __type_marker: typeof ARRAY_MARKER,
    item_type: {
        __type_marker: typeof INT_MARKER
    },
}

export const values_type: Values = {
    __type_marker: ARRAY_MARKER,
    item_type: { __type_marker: INT_MARKER },
};

const _values_test: ArrayField = values_type
