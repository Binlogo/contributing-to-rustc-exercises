//@ check-pass
//! This is the reduced version of the "Linux kernel vtable" use-case.

use std::ptr::addr_of_mut;

#[repr(C)]
struct ThisModule(i32);

trait Module {
    const THIS_MODULE_PTR: *mut ThisModule;
}

struct MyModule;

// Generated by a macro.
extern "C" {
    static mut THIS_MODULE: ThisModule;
}

// Generated by a macro.
impl Module for MyModule {
    const THIS_MODULE_PTR: *mut ThisModule = unsafe { addr_of_mut!(THIS_MODULE) };
}

struct Vtable {
    module: *mut ThisModule,
    foo_fn: fn(*mut ()) -> i32,
}

trait Foo {
    type Mod: Module;

    fn foo(&mut self) -> i32;
}

fn generate_vtable<T: Foo>() -> &'static Vtable {
    &Vtable {
        module: T::Mod::THIS_MODULE_PTR,
        foo_fn: |ptr| unsafe { &mut *ptr.cast::<T>() }.foo(),
    }
}

fn main() {}