#![allow(dead_code)]

pub mod ts_files;

fn do_stuff() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        do_stuff();
    }
}
