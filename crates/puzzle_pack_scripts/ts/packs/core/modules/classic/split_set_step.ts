import { classic_mod } from "../classic_mod";
import { allowed_values, full_set, required_values } from "./deductions";

declare const quantor: any;
declare const set: any;
declare const cmp: any;
declare const int: any;
type TODO = any;
const todo = "TODO";

const split_fullset_step = classic_mod.step({
    name: "split_fullset",
    logic: (matcher: TODO, emitter: TODO) => {
        const set1 = matcher.pool.get_one(full_set)

        const allowed_values_set = matcher.pool.get_many(allowed_values)
        const values = matcher.pool.get_many(int)
        matcher.where(
            quantor.all((matcher: TODO) => {
                const value = matcher.get_one(allowed_values)
                matcher.where(set.do(value, "element of", allowed_values_set))

                matcher.require(set.do(value.cell, "element of", set1.cells))
                matcher.require(set.do(value.values, "subset of", values))
            })
        )
        matcher.require(cmp.do(
            values.size, "==", allowed_values_set.size
        ))

        const set2 = matcher.pool.get_one(full_set)
        matcher.require(
            cmp.do(set2.values, "==", values)
        )
        matcher.require(
            quantor.all((matcher: TODO) => {
                const value = matcher.get_one(allowed_values)
                matcher.where(set.do(value, "element of", allowed_values_set))

                matcher.require(set.do(value.cell, "element of", set2.cells))
            })
        )

        const set3 = matcher.pool.get_one(full_set)
        matcher.require(
            cmp.do(set3.values, "==", set.do(set1.values, "without", set2.values))
        )
        matcher.require(
            cmp.do(set3.values, "==", set.do(set1.cells, "without", set2.cells))
        )

        const outer_allowed_values_set = todo

        emitter.emit(set2, set3, outer_allowed_values_set)
    }
})
