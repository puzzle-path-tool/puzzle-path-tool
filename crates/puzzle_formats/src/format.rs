pub trait PuzzleFormat {
    // This trait specifies, how a format is loaded
    // conversion from / to string
    // conversion from / to our own format

    //TODO: Implement this trait for all 4 formats (Penpa and Sudokumaker can have stubs, that only return an Error, eg. Format not Supported)
    // Maybe even with a custom message:

    // Sudokumaker: Sudokumaker is not supported yet, instead export the puzzle to Sudokupad before loading
    // Penpa: Penpa is not supported yet, instead use the Online Tool (https://marktekfan.github.io/sudokupad-penpa-import/) to convert the puzzle before loading
}
