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

* To test the conjunction of multiple modules within a crate
* Located in a "tests" folder alongside the "src" folder
* No need to mark with the #[cfg(test)] attribute - assumed automatically

----

### Performance Tests

* By convention appended to the same file to be tested
* release by default (--debug to override)

---

## proptest

> The best tests are those you do not need to write yourself. - Tyler Neely
