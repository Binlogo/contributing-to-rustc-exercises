//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print a
// gdb-check:$1 = 1
// gdb-command:print b
// gdb-check:$2 = false
// gdb-command:continue

// gdb-command:print a
// gdb-check:$3 = 2
// gdb-command:print b
// gdb-check:$4 = 3
// gdb-command:print c
// gdb-check:$5 = 4
// gdb-command:continue

// gdb-command:print a
// gdb-check:$6 = 5
// gdb-command:print b
// gdb-check:$7 = (6, 7)
// gdb-command:continue

// gdb-command:print h
// gdb-check:$8 = 8
// gdb-command:print i
// gdb-check:$9 = destructured_fn_argument::Struct {a: 9, b: 10}
// gdb-command:print j
// gdb-check:$10 = 11
// gdb-command:continue

// gdb-command:print k
// gdb-check:$11 = 12
// gdb-command:print l
// gdb-check:$12 = 13
// gdb-command:continue

// gdb-command:print m
// gdb-check:$13 = 14
// gdb-command:print n
// gdb-check:$14 = 16
// gdb-command:continue

// gdb-command:print o
// gdb-check:$15 = 18
// gdb-command:continue

// gdb-command:print p
// gdb-check:$16 = 19
// gdb-command:print q
// gdb-check:$17 = 20
// gdb-command:print r
// gdb-check:$18 = destructured_fn_argument::Struct {a: 21, b: 22}
// gdb-command:continue

// gdb-command:print s
// gdb-check:$19 = 24
// gdb-command:print t
// gdb-check:$20 = 23
// gdb-command:continue

// gdb-command:print u
// gdb-check:$21 = 25
// gdb-command:print v
// gdb-check:$22 = 26
// gdb-command:print w
// gdb-check:$23 = 27
// gdb-command:print x
// gdb-check:$24 = 28
// gdb-command:print y
// gdb-check:$25 = 29
// gdb-command:print z
// gdb-check:$26 = 30
// gdb-command:print ae
// gdb-check:$27 = 31
// gdb-command:print oe
// gdb-check:$28 = 32
// gdb-command:print ue
// gdb-check:$29 = 33
// gdb-command:continue

// gdb-command:print aa
// gdb-check:$30 = (34, 35)
// gdb-command:continue

// gdb-command:print bb
// gdb-check:$31 = (36, 37)
// gdb-command:continue

// gdb-command:print cc
// gdb-check:$32 = 38
// gdb-command:continue

// gdb-command:print dd
// gdb-check:$33 = (40, 41, 42)
// gdb-command:continue

// gdb-command:print *ee
// gdb-check:$34 = (43, 44, 45)
// gdb-command:continue

// gdb-command:print *ff
// gdb-check:$35 = 46
// gdb-command:print gg
// gdb-check:$36 = (47, 48)
// gdb-command:continue

// gdb-command:print *hh
// gdb-check:$37 = 50
// gdb-command:continue

// gdb-command:print ii
// gdb-check:$38 = 51
// gdb-command:continue

// gdb-command:print *jj
// gdb-check:$39 = 52
// gdb-command:continue

// gdb-command:print kk
// gdb-check:$40 = 53
// gdb-command:print ll
// gdb-check:$41 = 54
// gdb-command:continue

// gdb-command:print mm
// gdb-check:$42 = 55
// gdb-command:print *nn
// gdb-check:$43 = 56
// gdb-command:continue

// gdb-command:print oo
// gdb-check:$44 = 57
// gdb-command:print pp
// gdb-check:$45 = 58
// gdb-command:print qq
// gdb-check:$46 = 59
// gdb-command:continue

// gdb-command:print rr
// gdb-check:$47 = 60
// gdb-command:print ss
// gdb-check:$48 = 61
// gdb-command:print tt
// gdb-check:$49 = 62
// gdb-command:continue


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v a
// lldb-check:[...] 1
// lldb-command:v b
// lldb-check:[...] false
// lldb-command:continue

// lldb-command:v a
// lldb-check:[...] 2
// lldb-command:v b
// lldb-check:[...] 3
// lldb-command:v c
// lldb-check:[...] 4
// lldb-command:continue

// lldb-command:v a
// lldb-check:[...] 5
// lldb-command:v b
// lldb-check:[...] { 0 = 6 1 = 7 }
// lldb-command:continue

// lldb-command:v h
// lldb-check:[...] 8
// lldb-command:v i
// lldb-check:[...] { a = 9 b = 10 }
// lldb-command:v j
// lldb-check:[...] 11
// lldb-command:continue

// lldb-command:v k
// lldb-check:[...] 12
// lldb-command:v l
// lldb-check:[...] 13
// lldb-command:continue

// lldb-command:v m
// lldb-check:[...] 14
// lldb-command:v n
// lldb-check:[...] 16
// lldb-command:continue

// lldb-command:v o
// lldb-check:[...] 18
// lldb-command:continue

// lldb-command:v p
// lldb-check:[...] 19
// lldb-command:v q
// lldb-check:[...] 20
// lldb-command:v r
// lldb-check:[...] { a = 21 b = 22 }
// lldb-command:continue

// lldb-command:v s
// lldb-check:[...] 24
// lldb-command:v t
// lldb-check:[...] 23
// lldb-command:continue

// lldb-command:v u
// lldb-check:[...] 25
// lldb-command:v v
// lldb-check:[...] 26
// lldb-command:v w
// lldb-check:[...] 27
// lldb-command:v x
// lldb-check:[...] 28
// lldb-command:v y
// lldb-check:[...] 29
// lldb-command:v z
// lldb-check:[...] 30
// lldb-command:v ae
// lldb-check:[...] 31
// lldb-command:v oe
// lldb-check:[...] 32
// lldb-command:v ue
// lldb-check:[...] 33
// lldb-command:continue

// lldb-command:v aa
// lldb-check:[...] { 0 = 34 1 = 35 }
// lldb-command:continue

// lldb-command:v bb
// lldb-check:[...] { 0 = 36 1 = 37 }
// lldb-command:continue

// lldb-command:v cc
// lldb-check:[...] 38
// lldb-command:continue

// lldb-command:v dd
// lldb-check:[...] { 0 = 40 1 = 41 2 = 42 }
// lldb-command:continue

// lldb-command:v *ee
// lldb-check:[...] { 0 = 43 1 = 44 2 = 45 }
// lldb-command:continue

// lldb-command:v *ff
// lldb-check:[...] 46
// lldb-command:v gg
// lldb-check:[...] { 0 = 47 1 = 48 }
// lldb-command:continue

// lldb-command:v *hh
// lldb-check:[...] 50
// lldb-command:continue

// lldb-command:v ii
// lldb-check:[...] 51
// lldb-command:continue

// lldb-command:v *jj
// lldb-check:[...] 52
// lldb-command:continue

// lldb-command:v kk
// lldb-check:[...] 53
// lldb-command:v ll
// lldb-check:[...] 54
// lldb-command:continue

// lldb-command:v mm
// lldb-check:[...] 55
// lldb-command:v *nn
// lldb-check:[...] 56
// lldb-command:continue

// lldb-command:v oo
// lldb-check:[...] 57
// lldb-command:v pp
// lldb-check:[...] 58
// lldb-command:v qq
// lldb-check:[...] 59
// lldb-command:continue

// lldb-command:v rr
// lldb-check:[...] 60
// lldb-command:v ss
// lldb-check:[...] 61
// lldb-command:v tt
// lldb-check:[...] 62
// lldb-command:continue

#![allow(unused_variables)]
#![feature(box_patterns)]
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

use self::Univariant::Unit;

struct Struct {
    a: i64,
    b: i32
}

enum Univariant {
    Unit(i32)
}

struct TupleStruct (f64, isize);


fn simple_tuple((a, b): (isize, bool)) {
    zzz(); // #break
}

fn nested_tuple((a, (b, c)): (isize, (u16, u16))) {
    zzz(); // #break
}

fn destructure_only_first_level((a, b): (isize, (u32, u32))) {
    zzz(); // #break
}

fn struct_as_tuple_element((h, i, j): (i16, Struct, i16)) {
    zzz(); // #break
}

fn struct_pattern(Struct { a: k, b: l }: Struct) {
    zzz(); // #break
}

fn ignored_tuple_element((m, _, n): (isize, u16, i32)) {
    zzz(); // #break
}

fn ignored_struct_field(Struct { b: o, .. }: Struct) {
    zzz(); // #break
}

fn one_struct_destructured_one_not((Struct { a: p, b: q }, r): (Struct, Struct)) {
    zzz(); // #break
}

fn different_order_of_struct_fields(Struct { b: s, a: t }: Struct ) {
    zzz(); // #break
}

fn complex_nesting(((u,   v  ), ((w,   (x,   Struct { a: y, b: z})), Struct { a: ae, b: oe }), ue ):
                   ((i16, i32), ((i64, (i32, Struct,             )), Struct                 ), u16))
{
    zzz(); // #break
}

fn managed_box(&aa: &(isize, isize)) {
    zzz(); // #break
}

fn borrowed_pointer(&bb: &(isize, isize)) {
    zzz(); // #break
}

fn contained_borrowed_pointer((&cc, _): (&isize, isize)) {
    zzz(); // #break
}

fn unique_pointer(box dd: Box<(isize, isize, isize)>) {
    zzz(); // #break
}

fn ref_binding(ref ee: (isize, isize, isize)) {
    zzz(); // #break
}

fn ref_binding_in_tuple((ref ff, gg): (isize, (isize, isize))) {
    zzz(); // #break
}

fn ref_binding_in_struct(Struct { b: ref hh, .. }: Struct) {
    zzz(); // #break
}

fn univariant_enum(Unit(ii): Univariant) {
    zzz(); // #break
}

fn univariant_enum_with_ref_binding(Unit(ref jj): Univariant) {
    zzz(); // #break
}

fn tuple_struct(TupleStruct(kk, ll): TupleStruct) {
    zzz(); // #break
}

fn tuple_struct_with_ref_binding(TupleStruct(mm, ref nn): TupleStruct) {
    zzz(); // #break
}

fn multiple_arguments((oo, pp): (isize, isize), qq : isize) {
    zzz(); // #break
}

fn main() {
    simple_tuple((1, false));
    nested_tuple((2, (3, 4)));
    destructure_only_first_level((5, (6, 7)));
    struct_as_tuple_element((8, Struct { a: 9, b: 10 }, 11));
    struct_pattern(Struct { a: 12, b: 13 });
    ignored_tuple_element((14, 15, 16));
    ignored_struct_field(Struct { a: 17, b: 18 });
    one_struct_destructured_one_not((Struct { a: 19, b: 20 }, Struct { a: 21, b: 22 }));
    different_order_of_struct_fields(Struct { a: 23, b: 24 });
    complex_nesting(((25, 26), ((27, (28, Struct { a: 29, b: 30})), Struct { a: 31, b: 32 }), 33));
    managed_box(&(34, 35));
    borrowed_pointer(&(36, 37));
    contained_borrowed_pointer((&38, 39));
    unique_pointer(Box::new((40, 41, 42)));
    ref_binding((43, 44, 45));
    ref_binding_in_tuple((46, (47, 48)));
    ref_binding_in_struct(Struct { a: 49, b: 50 });
    univariant_enum(Unit(51));
    univariant_enum_with_ref_binding(Unit(52));
    tuple_struct(TupleStruct(53.0, 54));
    tuple_struct_with_ref_binding(TupleStruct(55.0, 56));
    multiple_arguments((57, 58), 59);

    fn nested_function(rr: isize, (ss, tt): (isize, isize)) {
        zzz(); // #break
    }

    nested_function(60, (61, 62));
}

fn zzz() { () }