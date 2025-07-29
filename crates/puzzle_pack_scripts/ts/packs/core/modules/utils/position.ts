import { obj, int, Var, Int } from "api/api";
import { utils_mod } from "../utils.mod";

export type PositionMathOp = "+" | "-";
export type PositionTransformOp = "mod" | "rem";

export const position = utils_mod.namespace({
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
