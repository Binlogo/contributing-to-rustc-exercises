//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print x
// gdb-check:$1 = false
// gdb-command:print y
// gdb-check:$2 = true
// gdb-command:continue

// gdb-command:print x
// gdb-check:$3 = 10
// gdb-command:print y
// gdb-check:$4 = true
// gdb-command:continue

// gdb-command:print x
// gdb-check:$5 = 10.5
// gdb-command:print y
// gdb-check:$6 = 20
// gdb-command:continue

// gdb-command:print x
// gdb-check:$7 = 10.5
// gdb-command:print y
// gdb-check:$8 = 20
// gdb-command:continue

// gdb-command:print x
// gdb-check:$9 = 11.5
// gdb-command:print y
// gdb-check:$10 = 20
// gdb-command:continue

// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v x
// lldb-check:[...] false
// lldb-command:v y
// lldb-check:[...] true
// lldb-command:continue

// lldb-command:v x
// lldb-check:[...] 10
// lldb-command:v y
// lldb-check:[...] true
// lldb-command:continue

// lldb-command:v x
// lldb-check:[...] 10.5
// lldb-command:v y
// lldb-check:[...] 20
// lldb-command:continue

// lldb-command:v x
// lldb-check:[...] 10.5
// lldb-command:v y
// lldb-check:[...] 20
// lldb-command:continue

// lldb-command:v x
// lldb-check:[...] 11.5
// lldb-command:v y
// lldb-check:[...] 20
// lldb-command:continue

#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

fn main() {
    let x = false;
    let y = true;

    zzz(); // #break
    sentinel();

    let x = 10;

    zzz(); // #break
    sentinel();

    let x = 10.5f64;
    let y = 20;

    zzz(); // #break
    sentinel();

    let x = {
        zzz(); // #break
        sentinel();
        11.5
    };

    zzz(); // #break
    sentinel()
}

fn zzz() {()}
fn sentinel() {()}