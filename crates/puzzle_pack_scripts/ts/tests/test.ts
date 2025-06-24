import * as a from "./opaque_test.js";
import { A } from "./opaque_test.js";

const x: A = {} as A;

a.doStuff({} as A);
