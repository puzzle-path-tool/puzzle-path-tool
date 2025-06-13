#![allow(dead_code)]

fn do_stuff() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_stuff();
    }
}
