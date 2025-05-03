#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Does Star Magic
#[must_use]
pub fn star_operation(left: u64, right: u64) -> u64 {
    (left + right) * left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
