
export type Rule<T> = { readonly __marker_rule__: unique symbol }

declare function create_rule<const T>(param: T): Rule<T> & Ordered & Deduplicated

export const x = create_rule({x: 1, y: [1, 2, "E"]})
//                            ^?


export type Unique = { readonly __marker_unique__: unique symbol }
export type Ordered = { readonly __marker_ordered__: unique symbol }
export type Deduplicated = { readonly __marker_deduplicated__: unique symbol }

const x2 = x
//     ^?
