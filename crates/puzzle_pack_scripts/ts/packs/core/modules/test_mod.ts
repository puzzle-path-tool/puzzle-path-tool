/* eslint-disable @typescript-eslint/no-explicit-any */
import { Decl, int, Int, obj, Obj, set, Set, Var } from "api/api";

import { core_pack } from "../core_pack";

declare const field: any;
declare const op: any;
declare const ty: any;
declare const step: any;
declare const dbg: any;
type TODO = any;
const todo = "TODO";

export const test_mod = core_pack.module({
    name: "test",
});

declare const fieldF: Decl<Obj<{ x: Int; y: Int }>>;

type PositionMathOp = "+" | "-";
type PositionTransformOp = "mod" | "rem";

export const position = test_mod.namespace({
    name: "position",
    description: "Position on the Grid",
    create: () => {
        const position = obj.decl({
            x: int.decl({ doc: "Horizontal (X) Position" }),
            y: int.decl({ doc: "Vertical (Y) Position" }),
        });
        const op = {
            math: (
                a: Var<typeof position.t>,
                o: PositionMathOp,
                b: Var<typeof position.t>,
            ): Var<typeof position.t> => {
                switch (o) {
                    case "+":
                        return {
                            x: int.op.math(a.x, "+", b.x),
                            y: int.op.math(a.y, "+", b.y),
                        };
                    case "-":
                        return {
                            x: int.op.math(a.x, "-", b.x),
                            y: int.op.math(a.y, "-", b.y),
                        };
                }
            },
            transform: (
                a: Var<typeof position.t>,
                o: PositionTransformOp,
                v: Var<Int>,
            ): Var<typeof position.t> => {
                switch (o) {
                    case "mod":
                        return {
                            x: int.op.math(a.x, "mod", v),
                            y: int.op.math(a.y, "mod", v),
                        };
                    case "rem":
                        return {
                            x: int.op.math(a.x, "rem", v),
                            y: int.op.math(a.y, "rem", v),
                        };
                }
            },
        };
        return {
            decl: position,
            op,
        };
    },
});

declare const fieldF3: Decl<
    Obj<{
        x: typeof position.t;
        y: typeof position.t;
    }>
>;

const pos1 = position.op.math({ x: 1, y: 1 }, "+", { x: 2, y: 2 });
//    ^?

const arrow = test_mod.deduction({
    name: "arrow",
    create: () => {
        const arrow = obj.decl({
            head: position.decl(),
            cells: set.decl(position.decl()),
        });
        const op = {
            f: () => {},
        };
        return {
            decl: arrow,
            op,
        };
    },
});

const full_set = test_mod.deduction({
    name: "full_set",
    create: () => ({
        decl: obj.decl({
            values: set.decl(int.decl()),
            cells: set.decl(position.decl()),
        }),
    }),
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
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        return step.define({
            condition: op.and(
                op.cmp(
                    op.set.sum((binding: TODO) => {
                        const value = binding.get_one(
                            full_set.type.values.item,
                        ); //TODO: this doesnt work

                        return value;
                    }),
                    "==",
                    value_sum,
                ),
                op.quantor.all((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return op.and(
                        op.set(value, "element of", set1.values), // TODO: This needs to be before colon, because of the "for all" x where A : B
                        // eg. A filters out
                        // B rejects the entire condition
                        op.cmp(value, ">", 1),
                        op.cmp(value, "<=", max_value),
                    );
                }),
                op.quantor.exists((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    // TODO: This is not bound
                    return op.and(op.cmp(value, "==", max_value));
                }),
            ),
            results: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

const step5 = test_mod.step({
    name: "step5",
    logic: (binding: TODO, step: TODO) => {
        const set1 = binding.get_one(full_set);
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        return step.define({
            condition: op.and(
                op.cmp(
                    op.set.sum((binding: TODO) => {
                        const value = binding.get_one(
                            full_set.type.values.item,
                        ); //TODO: this doesnt work

                        return value;
                    }),
                    "==",
                    value_sum,
                ),
                op.quantor.all((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return op.and(
                        op.set(value, "element of", set1.values), // TODO: This needs to be before colon, because of the "for all" x where A : B
                        // eg. A filters out
                        // B rejects the entire condition
                        op.cmp(value, ">", 1),
                        op.cmp(value, "<=", max_value),
                    );
                }),
                op.quantor.exists((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    // TODO: This is not bound
                    return op.and(op.cmp(value, "==", max_value));
                }),
            ),
            results: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

const step6 = test_mod.step({
    name: "step6",
    logic: (binding: TODO, step: TODO) => {
        const set1 = binding.get_one(full_set);
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        return step.define({
            where: op.exists(set1, max_value, value_sum),
            require: op.and(
                op.cmp(
                    op.set.sum(op.set.map(set1, (e: TODO) => e.values)),
                    "==",
                    value_sum,
                ),
                op.quantor.all((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return step.pack({
                        where: op.set(value, "element of", set1.values),
                        require: op.and(
                            op.cmp(value, ">", 1),
                            op.cmp(value, "<=", max_value),
                        ),
                    });
                }),
                op.quantor.exists((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return step.pack({
                        where: op.set(value, "element of", set1.values),
                        require: op.cmp(value, "==", max_value),
                    });
                }),
            ),
            emit: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

const step7 = test_mod.step({
    name: "step7",
    logic: (binding: TODO, step: TODO) => {
        const set1 = binding.get_one(full_set);
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        const values = op.set.map(set1, (e: TODO) => e.values);

        return step.define({
            where: op.exists(set1, max_value, value_sum),
            require: op.and(
                op.cmp(op.set.max(values), "==", max_value),
                op.cmp(op.set.sum(values), "==", value_sum),
            ),
            emit: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

const step8 = test_mod.step({
    name: "step8",
    logic: (binding: TODO, step: TODO) => {
        const set1 = binding.get_one(full_set);
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        const values = op.set.map(set1, (e: TODO) => e.values);

        return step.define({
            where: op.exists(set1, max_value, value_sum),
            require: op.and(
                op.cmp(
                    op.inline((binding: TODO) => {
                        const max_value = binding.get_one(int_type());

                        return step.pack_inline({
                            where: op.exists(max_value),
                            require: op.and(
                                op.quantor.all((binding: TODO) => {
                                    const value = binding.get_one(
                                        full_set.type.values.item,
                                    );

                                    return step.pack({
                                        where: op.set(
                                            value,
                                            "element of",
                                            set1.values,
                                        ),
                                        require: op.and(
                                            op.cmp(value, ">", 1),
                                            op.cmp(value, "<=", max_value),
                                        ),
                                    });
                                }),
                                op.quantor.exists((binding: TODO) => {
                                    const value = binding.get_one(
                                        full_set.type.values.item,
                                    );

                                    return step.pack({
                                        where: op.set(
                                            value,
                                            "element of",
                                            set1.values,
                                        ),
                                        require: op.cmp(value, "==", max_value),
                                    });
                                }),
                            ),
                            return: max_value,
                        });
                    }),
                    "==",
                    max_value,
                ),
                op.cmp(op.set.sum(values), "==", value_sum),
            ),
            emit: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

function custom_max(values: TODO): TODO {
    return op.inline((binding: TODO) => {
        const max_value = binding.get_one(int_type());

        return step.pack_inline({
            where: op.exists(max_value),
            require: op.and(
                op.quantor.all((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return step.pack({
                        where: op.set(value, "element of", values),
                        require: op.and(
                            op.cmp(value, ">", 1),
                            op.cmp(value, "<=", max_value),
                        ),
                    });
                }),
                op.quantor.exists((binding: TODO) => {
                    const value = binding.get_one(full_set.type.values.item);

                    return step.pack({
                        where: op.set(value, "element of", values),
                        require: op.cmp(value, "==", max_value),
                    });
                }),
            ),
            return: max_value,
        });
    });
}

const step9 = test_mod.step({
    name: "step9",
    logic: (binding: TODO) => {
        const set1 = binding.get_one(full_set);
        const max_value = binding.get_one(int_type());
        const value_sum = binding.get_one(int_type());

        const values = op.set.map(set1, (e: TODO) => e.values);

        return step.define({
            where: op.exists(set1, max_value, value_sum),
            require: op.and(
                op.cmp(custom_max(values), "==", max_value),
                op.cmp(op.set.sum(values), "==", value_sum),
            ),
            emit: [
                step.new(another_ded, {
                    max: max_value,
                    sum: value_sum,
                }),
            ],
        });
    },
});

const step10 = test_mod.step({
    name: "step10",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.get_one(full_set);
        matcher.where(op.exists(set1));

        const max_value = matcher.get_one(int_type());
        matcher.where(op.exists(max_value));

        const value_sum = matcher.get_one(int_type());
        matcher.where(op.exists(value_sum));

        matcher.require(op.cmp(op.set.sum(set1.values), "==", value_sum));

        matcher.require(
            op.quantor.all((matcher: TODO) => {
                const value = matcher.get_one(full_set.type.values.item);
                matcher.where(op.set(value, "element of", set1.values));

                matcher.require(op.cmp(value, ">", 1));
                matcher.require(op.cmp(value, "<=", max_value));
            }),
        );

        matcher.require(
            op.quantor.exists((matcher: TODO) => {
                const value = matcher.get_one(full_set.type.values.item);
                matcher.where(op.set(value, "element of", set1.values));

                matcher.require(op.cmp(value, "==", max_value));
            }),
        );

        emitter.emit_one(another_ded, {
            max: max_value,
            sum: value_sum,
        });
    },
});

const step11 = test_mod.step({
    name: "step11",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.get_one(full_set.type);
        matcher.where(op.pool(set1, "from", full_set));
        matcher.where(op.exists(set1));

        const max_value = matcher.get_one(ty.int());
        matcher.where(op.exists(max_value));

        const value_sum = matcher.get_one(ty.int());
        matcher.where(op.exists(value_sum));

        matcher.require(op.cmp(op.set.max(set1.values), "==", max_value));
        matcher.require(op.cmp(op.set.sum(set1.values), "==", value_sum));

        emitter.emit_one(another_ded, {
            max: max_value,
            sum: value_sum,
        });
    },
});

const step12 = test_mod.step({
    name: "step12",
    logic: (matcher: TODO) => {
        const set1 = matcher.get_one(full_set);
        matcher.where(op.exists(set1));

        const max_value = matcher.get_one(int_type());
        matcher.where(op.exists(max_value));

        const value_sum = matcher.get_one(int_type());
        matcher.where(op.exists(value_sum));

        matcher.require(op.cmp(op.set.max(set1.values), "==", max_value));
        matcher.require(op.cmp(op.set.sum(set1.values), "==", value_sum));

        return step.emit(
            step.new(another_ded, {
                max: max_value,
                sum: value_sum,
            }),
        );
    },
});

const step13 = test_mod.step({
    name: "step13",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.get_one(ty.of(full_set));
        matcher.where(op.pool(set1, "from", full_set));

        const max_value = matcher.get_one(ty.int());
        matcher.where(op.cmp(op.set.max(set1.values), "==", max_value));

        const value_sum = matcher.get_one(ty.int());
        matcher.where(op.cmp(op.set.sum(set1.values), "==", value_sum));

        matcher.require(op.cmp(op.int(max_value, "*", 3), ">", value_sum));

        emitter.emit_one(another_ded, {
            max: max_value,
            sum: value_sum,
        });
    },
});

const step14 = test_mod.step({
    name: "step14",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.get_one(ty.of(full_set));
        matcher.where(op.pool(set1, "from", full_set));

        const max_value = matcher.get_one(ty.int());
        matcher.where(op.set(max_value, "element of", set1.values));
        matcher.where(
            op.quantor.all((matcher: TODO) => {
                const value = matcher.get_one(ty.item(ty.of(full_set).values));
                matcher.where(op.set(value, "element of", set1.values));

                matcher.require(op.cmp(value, "<=", max_value));
            }),
        );

        const value_sum = matcher.get_one(ty.int());
        matcher.where(op.cmp(op.set.sum(set1.values), "==", value_sum));

        matcher.require(op.cmp(op.int(max_value, "*", 3), ">", value_sum));

        emitter.emit(
            another_ded,
            {
                max: max_value,
                sum: value_sum,
            },
            {
                max: max_value,
                sum: 1,
            },
        );
    },
});

const step15 = test_mod.step({
    name: "step15",
    logic: (matcher, emitter) => {
        const set1a = matcher.pool.get_one(full_set);
        const set1 = matcher.pool.get_one(full_set);
        matcher.debugSymbols({ set1a, set1 });

        const max_value = matcher.get_one(int.t);
        matcher.where(
            int.op.cmp(int.op.fold("max", set1.values), "==", max_value),
        );

        const value_sum = matcher.get_one(int.t);
        matcher.where(
            int.op.cmp(int.op.fold("+", set1.values), "==", value_sum),
        );

        matcher.require(
            int.op.cmp(int.op.math(max_value, "*", 3), ">", value_sum),
        );

        // const output = {
        //     max: max_value,
        //     sum: value_sum,
        // };

        // matcher.require(another_ded.op.fold(""));
        // matcher.require(another_ded.invariant.something(output));

        emitter.emit_one(another_ded, {
            cells: set1a.cells,
            values: set1.values,
        });
    },
});
