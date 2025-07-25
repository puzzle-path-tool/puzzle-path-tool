// type X<A, B, C> = (A extends undefined ? unknown : { a: A }) &
//     (B extends undefined ? unknown : { b: B }) &
//     (C extends undefined ? unknown : { c: C });

// declare const x: X<number, string, undefined>;

type X<A, B, C> = ([A] extends [never] ? unknown : { a: A }) &
    ([B] extends [never] ? unknown : { b: B }) &
    ([C] extends [never] ? unknown : { c: C });

declare const x: X<number, string, never>;
