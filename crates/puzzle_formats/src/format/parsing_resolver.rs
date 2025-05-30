use super::{full::FullPuzzle, logical::LogicalPuzzle, visual::VisualPuzzle};

pub trait ParsingResolver {
    fn construct_logical(full: FullPuzzle) -> LogicalPuzzle;
    fn construct_visual(full: FullPuzzle) -> VisualPuzzle;
    fn from_logical(logical: LogicalPuzzle) -> FullPuzzle;
}

//TODO, add ability to fail, and stepwise transformation (Maybe Some kind of Builder?)

// Puzzle Operations:

// fn construct_logical(full: FullPuzzle) -> LogicalPuzzle;
// fn construct_visual(full: FullPuzzle) -> VisualPuzzle;
// fn from_logical(logical: LogicalPuzzle) -> FullPuzzle;
