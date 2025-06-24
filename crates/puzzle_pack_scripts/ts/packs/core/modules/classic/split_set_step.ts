import type { Match, SetInput } from "packs/api/api";
import type { AllowedValue, FullSet } from "./deductions";

const split_full_match: Match<[FullSet, AllowedValue[]]> = {
    operator: "AND",
    first: {
        // cells_a.map(cell) ⊂ set_a.cells
        operator: "IS_TRUE_SUBSET",
        input: {
            first: {
                d: "SET_MAP",
            },
            second: {
                index: 0,
                path: ["fields", "cells"],
                d: "SET_PATH",
            },
        },
    },
    second: {
        // cells_a.union(x -> x.values) == cells_a.count()
        operator: "EQUALS",
        first: { count: { operator: "UNION", input: { d: "SET_MAP" } } },
        second: {
            count: {
                index: 0,
                path: ["field"],
                d: "SET_PATH",
            },
        },
    },
};
