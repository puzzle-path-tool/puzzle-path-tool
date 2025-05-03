///
/// This Does some things
///
#[unsafe(no_mangle)]
pub extern "C" fn star_operation(left: u64, right: u64) -> u64 {
    puzzle_formats::star_operation(left, right)
}

/// Does Wild Magic
#[unsafe(no_mangle)]
pub extern "C" fn star_operation23(left: u64, right: u64) -> u64 {
    puzzle_formats::star_operation(left, right)
}
