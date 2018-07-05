---
title: Testing in Rust
revealOptions:
    transition: 'slide'
---

# Testing in Rust

* Language Feature
* Specialized Libraries
  * e.g. for Property Testing
* Language-agnostic tools
  * e.g. Jepsen, TLA+

Note: speaker notes FTW!

---

## Testing Language Features

* Unit Tests
* Integration Tests
* Performance Tests

----

### The "test" build configuration

* Build/run using "cargo test"
* *Not* built with "cargo build"
* Mark test-only code sections using the #[cfg(test)] attribute
* Only compiled in test configurations

----

### Unit Tests

* To test a *single* module
* By convention appended to the same file to be tested
* By convention in a submodule named "tests"
* Simply mark a Rust function with the [#test] attribute
* Debug configuration by default (--release to override)
* Rust allows testing of private functions

----

### Integration Tests

* To test multiple modules in combination
* Located in a "tests" folder alongside the "src" folder
* No need to mark with the #[cfg(test)] attribute
* Tested crate needs to be imported
* Each integration test source file compiled as separate crate

----

### Test Filter Options

* Run a specific unit/performance test specifying the test name
  * Name needs to include the complete module path
  * Module path alone runs all tests below that module
* Run a specific integration test using "--test <filename.rs>"

----

### Performance Tests

* Requires the "test" feature and crate import
```
#![feature(test)]
extern crate test;
```
* Run with "cargo bench"
* Use the "#[bench]" attribute on the test function
---

## proptest

> The best tests are those you do not need to write yourself. - Tyler Neely

----

* Based on Python's "Hypothesis" module
  * Which in turn is based on Haskell's "QuickCheck"
* By convention appended to the same file to be tested
