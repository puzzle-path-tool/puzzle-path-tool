#![allow(dead_code)]

pub mod files;

fn do_stuff() {}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_stuff();
    }
}
