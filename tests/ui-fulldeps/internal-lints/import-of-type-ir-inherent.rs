//@ compile-flags: -Z unstable-options

// #[cfg(bootstrap)]: We can stop ignoring next beta bump; afterward this ALWAYS should run.
//@ ignore-stage1

#![feature(rustc_private)]
#![deny(rustc::usage_of_type_ir_inherent)]

extern crate rustc_type_ir;

use rustc_type_ir::inherent::*;
//~^ ERROR do not use `rustc_type_ir::inherent` unless you're inside of the trait solver
use rustc_type_ir::inherent;
//~^ ERROR do not use `rustc_type_ir::inherent` unless you're inside of the trait solver
use rustc_type_ir::inherent::Predicate;
//~^ ERROR do not use `rustc_type_ir::inherent` unless you're inside of the trait solver

fn main() {}