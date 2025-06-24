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

class Op {
    static cmp<T>(a: T, op: "==", b: T): boolean;
    static cmp(a: unknown, op: CmpOp, b: unknown): unknown {
        return a;
    }
}

export const op = Op;
