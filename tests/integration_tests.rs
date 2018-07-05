extern crate rust_testing;

use rust_testing::a_public_function;

// private functions fail to import in integration tests!
//use rust_testing::a_private_function;

#[test]
fn integration_test() {
    assert_eq!(a_public_function(), 1);    
}