// Make sure that `#[expect(missing_docs)]` is always correctly fulfilled.

//@ check-pass
//@ revisions: lib bin test
//@ [lib]compile-flags: --crate-type lib
//@ [bin]compile-flags: --crate-type bin
//@ [test]compile-flags: --test

#[expect(missing_docs)]
pub fn foo() {}

#[cfg(bin)]
fn main() {}