#![feature(test)]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate proptest;

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

    proptest! {
        #[test]
        fn a_proptest(a in (0i32..100),
                      b in (0i32..100)) {
            assert!(a * b <= 10000);
        }
    }
}
