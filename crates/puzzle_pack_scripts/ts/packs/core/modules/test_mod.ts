/* eslint-disable @typescript-eslint/no-explicit-any */
import { core_pack } from "../core_pack";

declare const Field: any;
declare const op: any;
type TODO = any;

type SetOp =
    | "subset of"
    | "superset of"
    | "true subset of"
    | "true superset of"
    | "element of"
    | "contains"
    | "union"
    | "intersect"
    | "without"
    | "subtracted from"
    | "disjoint with"
    | "disjunctive union";

type CmpOp = "==" | "!=" | ">=" | "<=" | ">" | "<";

type IntOp = "+" | "-" | "*" | "//" | "mod" | "rem" | "**";
type BitOp = "&" | "|" | "^" | ">>" | "<<" | ">>>" | "<<<" | ">>>";

export const test_mod = core_pack.module({
    name: "test",
});

export function position(props?: TODO) {
    return Field.object(
        {
            x: Field.int(),
            y: Field.int(),
        },
        props,
    );
}

const arrow = test_mod.deduction({
    name: "arrow",
    data: {
        head: position(),
        cells: Field.set(position(), {
            ordered: true,
        }),
    },
});

const full_set = test_mod.deduction({
    name: "full_set",
    data: {
        values: Field.set(Field.int()),
        cells: Field.set(position()),
    },
});

const step1 = test_mod.step({
    name: "step1",
    logic: (step: TODO) => {
        const large_set = step.get_one(full_set);
        const small_set = step.get_one(full_set);

        const cell = step.get_field(position());

        const large_only_cells = op.set(
            large_set.cells,
            "without",
            small_set.cells,
        );

        const large_only_values = op.set(
            large_set.values,
            "without",
            small_set.values,
        );

        return step.define({
            condition: op.and(
                op.set(small_set.cells, "subset of", large_set.cells),
                op.cmp(op.len(large_only_cells), ">=", 1),
            ),
            results: [
                step.new(full_set, {
                    values: large_only_values,
                    cells: large_only_cells,
                }),
            ],
        });
    },
});

const step2 = test_mod.step({
    name: "step2",
    logic: (step: TODO) => {
        const large_set = step.get_one(full_set);
        const small_set = step.get_one(full_set);

        const cell = step.get_field(position());

        const large_only_cells = op.set(
            large_set.cells,
            "without",
            small_set.cells,
        );

        const large_only_values = op.set(
            large_set.values,
            "without",
            small_set.values,
        );

        return step.define({
            condition: op.and(
                op.set(small_set.cells, "subset of", large_set.cells),
                op.cmp(op.len(large_only_cells), ">=", 1),
            ),
            results: [
                step.new(full_set, {
                    values: large_only_values,
                    cells: large_only_cells,
                }),
            ],
        });
    },
});
