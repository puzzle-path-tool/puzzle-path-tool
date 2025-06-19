import { do_stuff } from "packs/api/api";
import type { ExamplePuzzptApi } from "packs/api/puzzpt_api";

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
