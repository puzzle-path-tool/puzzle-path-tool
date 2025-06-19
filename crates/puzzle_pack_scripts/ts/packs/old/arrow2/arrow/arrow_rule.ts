import { arrowMod } from "../arrow_mod";

// const Field: any = {};

// export const arrowRule = arrowMod.createDeduction({
//     data: {
//         head: Field.position(),
//         cells: Field.array({
//             item: Field.object({
//                 position: Field.position(),
//                 value: Field.int(),
//             }),
//             opt1: true,
//         }),
//     },
// });

// export const arrowRule2 = arrowMod.createDeduction({
//     data: {
//         head: Field.position(),
//         cells: Field.array(
//             Field.object(
//                 {
//                     position: Field.position(),
//                     value: Field.int({ range: "sudoku" }),
//                 },
//                 { opt1: true },
//             ),
//             {
//                 opt1: true,
//             },
//         ),
//     },
// });

// arrowRule2.f.cells.position

// ///

// export const puzzle = createPuzzle((puzzle) => {
//     puzzle.build()

//     puzzle.build()

//     puzzle.build()
// })

// ///

// puzzle = createPuzzle({...});

// export const puzzle1 = puzzle.build();

// ///
// export const puzzle2 = puzzle.build();

///

//puzzleA /// A -> B14 -> C

//puzzleB /// extract => B14 -> ...
