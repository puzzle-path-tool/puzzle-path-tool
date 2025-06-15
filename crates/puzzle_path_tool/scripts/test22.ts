import { do_stuff } from "./api/api";
import type { ExamplePuzzptApi } from "./api/puzzpt_api.generated";

type A = 1;

const x: ExamplePuzzptApi = {
    value: "",
    number: 1,
    number2: 3,
};

const y = do_stuff(x);
