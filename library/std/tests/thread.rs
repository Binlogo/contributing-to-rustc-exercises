use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
#[cfg_attr(miri, ignore)] // Miri does not like the thread leak
fn sleep_very_long() {
    let finished = Arc::new(Mutex::new(false));
    let t_finished = finished.clone();
    thread::spawn(move || {
        thread::sleep(Duration::new(u64::MAX, 0));
        *t_finished.lock().unwrap() = true;
    });
    thread::sleep(Duration::from_millis(100));
    assert_eq!(*finished.lock().unwrap(), false);
}

#[test]
fn thread_local_containing_const_statements() {
    // This exercises the `const $init:block` cases of the thread_local macro.
    // Despite overlapping with expression syntax, the `const { ... }` is not
    // parsed as `$init:expr`.
    thread_local! {
        static CELL: Cell<u32> = const {
            let value = 1;
            Cell::new(value)
        };

        static REFCELL: RefCell<u32> = const {
            let value = 1;
            RefCell::new(value)
        };
    }

    assert_eq!(CELL.get(), 1);
    assert_eq!(REFCELL.take(), 1);
}

#[test]
// Include an ignore list on purpose, so that new platforms don't miss it
#[cfg_attr(
    any(
        target_os = "redox",
        target_os = "l4re",
        target_env = "sgx",
        target_os = "solid_asp3",
        target_os = "teeos",
        target_os = "wasi"
    ),
    should_panic
)]
fn available_parallelism() {
    // check that std::thread::available_parallelism() returns a valid value
    assert!(thread::available_parallelism().is_ok());
}