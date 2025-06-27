/* eslint-disable @typescript-eslint/no-explicit-any */
import { core_pack } from "../core_pack";

declare const Field: any;
declare const op: any;
type TODO = any;
const todo = "TODO";

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

const another_ded = full_set;

const step1 = test_mod.step({
    name: "step1",
    logic: (step: TODO) => {
        const large_set = step.get_one(full_set);
        const small_set = step.get_one(full_set);

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
        // const setB1 = step.get_one(another_ded);

        const set1 = step.get_one(full_set);
        const set2 = step.get_one(full_set);
        const set3 = step.get_one(full_set);

        // const set4 = step.get_none(full_set);

        // const cell1 = step.get_one(full_set.type.cells.item); // {x: int, y: int}
        const cell1 = step.get_one(position()); // {x: int, y: int}
        const cell2 = cell1.map((c: any) => ({
            x: op.int(c.x, "+", 1),
            y: c.y,
        })); // {x: int, y: int}

        return step.define({
            condition: op.and(
                true, //
                op.set(cell1, "element of", set1.cells),
                op.set(cell1, "element of", set2.cells),
                op.set(cell1, "element of", set3.cells),
                op.not(op.set(cell2, "element of", set3.cells)),
                op.different(set1, set2, set3),
                // op.set(cell1, "element of", setB1.cells),
            ),
            results: [
                step.new(full_set, {
                    values: todo,
                    cells: todo,
                }),
            ],
        });
    },
});

const step3 = test_mod.step({
    name: "step2",
    logic: (step: TODO) => {
        // const setB1 = step.get_one(another_ded);

        const set1 = step.get_one(full_set);
        const set2 = step.get_one(full_set);
        const set3 = step.get_one(full_set);

        // const set4 = step.get_none(full_set);

        // const cell1 = step.get_one(full_set.type.cells.item); // {x: int, y: int}
        const cell1 = step.get_one(position()); // {x: int, y: int}
        const cell2 = cell1.map((c: any) => ({
            x: op.int(c.x, "+", 1),
            y: c.y,
        })); // {x: int, y: int}

        op.quantor.exists((binding: TODO) => {
            const set1 = binding.get_one(full_set);

            return op.quantor.all((binding: TODO) => {
                const value = binding.get_one(full_set.type.values.item);

                return op.and(
                    op.set(value, "element of", set1.values),
                    op.cmp(value, ">", 1),
                );
            });
        });

        step.exists((s: TODO) => {
            const set1 = s.get_one(full_set);

            return step.condition(
                step.all(() => {
                    const value = set1.get_element(full_set.type.values.item);

                    return step.condition(op.cmp(value, ">", 1));
                }),
            );
        });

        step.exists((s: TODO) => {
            const set1 = s.get_one(full_set);
            const set2 = s.get_one(full_set);

            return step.condition(
                op.and(
                    op.different(set1, set2),
                    step.all(() => {
                        const value = op
                            .set(set1, "union", set2)
                            .get_element(full_set.type.values.item);

                        return step.condition(op.cmp(value, ">", 1));
                    }),
                ),
            );
        });

        return step.define({
            condition: op.and(
                true, //
                op.set(cell1, "element of", set1.cells),
                op.set(cell1, "element of", set2.cells),
                op.set(cell1, "element of", set3.cells),
                op.not(op.set(cell2, "element of", set3.cells)),
                op.different(set1, set2, set3),
                // op.set(cell1, "element of", setB1.cells),
            ),
            results: [
                step.new(full_set, {
                    values: todo,
                    cells: todo,
                }),
            ],
        });
    },
});

const step4 = test_mod.step({
    name: "step4",
    logic: (binding: TODO, step: TODO) => {
        const set1 = binding.get_one(full_set);

        return step.define({
            condition: op.quantor.all((binding: TODO) => {
                const value = binding.get_one(full_set.type.values.item);

                return op.and(
                    op.set(value, "element of", set1.values),
                    op.cmp(value, ">", 1),
                );
            }),
            results: [
                step.new(full_set, {
                    values: set1.values,
                    cells: set1.cells,
                }),
            ],
        });
    },
});
