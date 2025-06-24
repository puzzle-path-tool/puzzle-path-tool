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
