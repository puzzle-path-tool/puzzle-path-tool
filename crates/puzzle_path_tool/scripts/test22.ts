import { do_stuff } from "./api/api";
import type { ExamplePuzzptApi } from "./api/puzzpt_api.generated";

type A = 1;

const x: ExamplePuzzptApi = {
    export_tag: "ExamplePuzzptApi",
    value: "",
    number: 1,
    stuff: { tag: "StuffA", value: 1 },
};

class RuleTest {
    private puzzpt_export: ExamplePuzzptApi;
    constructor(v: ExamplePuzzptApi) {
        this.puzzpt_export = v;
    }
}

export const ruleTest = new RuleTest(x);

const y = do_stuff(x);
