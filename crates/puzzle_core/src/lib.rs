#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod explorer_collection;

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
