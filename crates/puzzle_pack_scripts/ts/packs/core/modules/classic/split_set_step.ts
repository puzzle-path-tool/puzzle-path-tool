import type { Match } from "packs/api/api";
import type { AllowedValue, FullSet } from "./deductions";

const split_full_match: Match<[AllowedValue[], FullSet]> = {
    operator: "AND",
    first: {
        // cells_a.map(cell) ⊂ set_a.cells
        operator: "IS_TRUE_SUBSET",
        first: {
            d: "SET_MAP",
        },
        second: {
            index: 1,
            path: ["cells"]
        },
    },
    second: {
        // cells_a.union(x -> x.values) == cells_a.count()
        operator: "EQUALS",
        first: { count: { operator: "UNION", input: { d: "SET_MAP" } } },
        second: {
            count: {
                d: "SET_MAP"
            },
        },
    },
};
