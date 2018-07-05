#![feature(test)]
extern crate test;

mod child_module;

pub fn a_public_function() -> i32 {
    return 1;
}

#[cfg(test)]
fn a_private_function() -> i32 {
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn hello() {
        assert_eq!(a_private_function(), 1);
        assert_eq!(a_public_function(), 1);
    }

    #[bench]
    fn a_benchmark(b: &mut Bencher) {
        b.iter(|| a_public_function());
    }
}
