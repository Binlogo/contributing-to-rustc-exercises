//! Code that is useful in various codegen modules.

use libc::{c_char, c_uint};
use rustc_abi as abi;
use rustc_abi::Primitive::Pointer;
use rustc_abi::{AddressSpace, HasDataLayout};
use rustc_ast::Mutability;
use rustc_codegen_ssa::traits::*;
use rustc_data_structures::stable_hasher::{Hash128, HashStable, StableHasher};
use rustc_hir::def_id::DefId;
use rustc_middle::bug;
use rustc_middle::mir::interpret::{ConstAllocation, GlobalAlloc, Scalar};
use rustc_middle::ty::TyCtxt;
use rustc_session::cstore::DllImport;
use tracing::debug;

use crate::consts::const_alloc_to_llvm;
pub(crate) use crate::context::CodegenCx;
use crate::llvm::{self, BasicBlock, Bool, ConstantInt, False, Metadata, OperandBundleDef, True};
use crate::type_::Type;
use crate::value::Value;

/*
* A note on nomenclature of linking: "extern", "foreign", and "upcall".
*
* An "extern" is an LLVM symbol we wind up emitting an undefined external
* reference to. This means "we don't have the thing in this compilation unit,
* please make sure you link it in at runtime". This could be a reference to
* C code found in a C library, or rust code found in a rust crate.
*
* Most "externs" are implicitly declared (automatically) as a result of a
* user declaring an extern _module_ dependency; this causes the rust driver
* to locate an extern crate, scan its compilation metadata, and emit extern
* declarations for any symbols used by the declaring crate.
*
* A "foreign" is an extern that references C (or other non-rust ABI) code.
* There is no metadata to scan for extern references so in these cases either
* a header-digester like bindgen, or manual function prototypes, have to
* serve as declarators. So these are usually given explicitly as prototype
* declarations, in rust code, with ABI attributes on them noting which ABI to
* link via.
*
* An "upcall" is a foreign call generated by the compiler (not corresponding
* to any user-written call in the code) into the runtime library, to perform
* some helper task such as bringing a task to life, allocating memory, etc.
*
*/

/// A structure representing an active landing pad for the duration of a basic
/// block.
///
/// Each `Block` may contain an instance of this, indicating whether the block
/// is part of a landing pad or not. This is used to make decision about whether
/// to emit `invoke` instructions (e.g., in a landing pad we don't continue to
/// use `invoke`) and also about various function call metadata.
///
/// For GNU exceptions (`landingpad` + `resume` instructions) this structure is
/// just a bunch of `None` instances (not too interesting), but for MSVC
/// exceptions (`cleanuppad` + `cleanupret` instructions) this contains data.
/// When inside of a landing pad, each function call in LLVM IR needs to be
/// annotated with which landing pad it's a part of. This is accomplished via
/// the `OperandBundleDef` value created for MSVC landing pads.
pub(crate) struct Funclet<'ll> {
    cleanuppad: &'ll Value,
    operand: OperandBundleDef<'ll>,
}

impl<'ll> Funclet<'ll> {
    pub(crate) fn new(cleanuppad: &'ll Value) -> Self {
        Funclet { cleanuppad, operand: OperandBundleDef::new("funclet", &[cleanuppad]) }
    }

    pub(crate) fn cleanuppad(&self) -> &'ll Value {
        self.cleanuppad
    }

    pub(crate) fn bundle(&self) -> &OperandBundleDef<'ll> {
        &self.operand
    }
}

impl<'ll> BackendTypes for CodegenCx<'ll, '_> {
    type Value = &'ll Value;
    type Metadata = &'ll Metadata;
    // FIXME(eddyb) replace this with a `Function` "subclass" of `Value`.
    type Function = &'ll Value;

    type BasicBlock = &'ll BasicBlock;
    type Type = &'ll Type;
    type Funclet = Funclet<'ll>;

    type DIScope = &'ll llvm::debuginfo::DIScope;
    type DILocation = &'ll llvm::debuginfo::DILocation;
    type DIVariable = &'ll llvm::debuginfo::DIVariable;
}

impl<'ll> CodegenCx<'ll, '_> {
    pub(crate) fn const_array(&self, ty: &'ll Type, elts: &[&'ll Value]) -> &'ll Value {
        let len = u64::try_from(elts.len()).expect("LLVMConstArray2 elements len overflow");
        unsafe { llvm::LLVMConstArray2(ty, elts.as_ptr(), len) }
    }

    pub(crate) fn const_bytes(&self, bytes: &[u8]) -> &'ll Value {
        bytes_in_context(self.llcx, bytes)
    }

    pub(crate) fn const_get_elt(&self, v: &'ll Value, idx: u64) -> &'ll Value {
        unsafe {
            let idx = c_uint::try_from(idx).expect("LLVMGetAggregateElement index overflow");
            let r = llvm::LLVMGetAggregateElement(v, idx).unwrap();

            debug!("const_get_elt(v={:?}, idx={}, r={:?})", v, idx, r);

            r
        }
    }
}

impl<'ll, 'tcx> ConstCodegenMethods<'tcx> for CodegenCx<'ll, 'tcx> {
    fn const_null(&self, t: &'ll Type) -> &'ll Value {
        unsafe { llvm::LLVMConstNull(t) }
    }

    fn const_undef(&self, t: &'ll Type) -> &'ll Value {
        unsafe { llvm::LLVMGetUndef(t) }
    }

    fn const_poison(&self, t: &'ll Type) -> &'ll Value {
        unsafe { llvm::LLVMGetPoison(t) }
    }

    fn const_bool(&self, val: bool) -> &'ll Value {
        self.const_uint(self.type_i1(), val as u64)
    }

    fn const_i8(&self, i: i8) -> &'ll Value {
        self.const_int(self.type_i8(), i as i64)
    }

    fn const_i16(&self, i: i16) -> &'ll Value {
        self.const_int(self.type_i16(), i as i64)
    }

    fn const_i32(&self, i: i32) -> &'ll Value {
        self.const_int(self.type_i32(), i as i64)
    }

    fn const_int(&self, t: &'ll Type, i: i64) -> &'ll Value {
        unsafe { llvm::LLVMConstInt(t, i as u64, True) }
    }

    fn const_u8(&self, i: u8) -> &'ll Value {
        self.const_uint(self.type_i8(), i as u64)
    }

    fn const_u32(&self, i: u32) -> &'ll Value {
        self.const_uint(self.type_i32(), i as u64)
    }

    fn const_u64(&self, i: u64) -> &'ll Value {
        self.const_uint(self.type_i64(), i)
    }

    fn const_u128(&self, i: u128) -> &'ll Value {
        self.const_uint_big(self.type_i128(), i)
    }

    fn const_usize(&self, i: u64) -> &'ll Value {
        let bit_size = self.data_layout().pointer_size.bits();
        if bit_size < 64 {
            // make sure it doesn't overflow
            assert!(i < (1 << bit_size));
        }

        self.const_uint(self.isize_ty, i)
    }

    fn const_uint(&self, t: &'ll Type, i: u64) -> &'ll Value {
        unsafe { llvm::LLVMConstInt(t, i, False) }
    }

    fn const_uint_big(&self, t: &'ll Type, u: u128) -> &'ll Value {
        unsafe {
            let words = [u as u64, (u >> 64) as u64];
            llvm::LLVMConstIntOfArbitraryPrecision(t, 2, words.as_ptr())
        }
    }

    fn const_real(&self, t: &'ll Type, val: f64) -> &'ll Value {
        unsafe { llvm::LLVMConstReal(t, val) }
    }

    fn const_str(&self, s: &str) -> (&'ll Value, &'ll Value) {
        let str_global = *self
            .const_str_cache
            .borrow_mut()
            .raw_entry_mut()
            .from_key(s)
            .or_insert_with(|| {
                let sc = self.const_bytes(s.as_bytes());
                let sym = self.generate_local_symbol_name("str");
                let g = self.define_global(&sym, self.val_ty(sc)).unwrap_or_else(|| {
                    bug!("symbol `{}` is already defined", sym);
                });
                unsafe {
                    llvm::LLVMSetInitializer(g, sc);
                    llvm::LLVMSetGlobalConstant(g, True);
                    llvm::LLVMSetUnnamedAddress(g, llvm::UnnamedAddr::Global);
                    llvm::LLVMRustSetLinkage(g, llvm::Linkage::InternalLinkage);
                }
                (s.to_owned(), g)
            })
            .1;
        let len = s.len();
        (str_global, self.const_usize(len as u64))
    }

    fn const_struct(&self, elts: &[&'ll Value], packed: bool) -> &'ll Value {
        struct_in_context(self.llcx, elts, packed)
    }

    fn const_vector(&self, elts: &[&'ll Value]) -> &'ll Value {
        let len = c_uint::try_from(elts.len()).expect("LLVMConstVector elements len overflow");
        unsafe { llvm::LLVMConstVector(elts.as_ptr(), len) }
    }

    fn const_to_opt_uint(&self, v: &'ll Value) -> Option<u64> {
        try_as_const_integral(v).and_then(|v| unsafe {
            let mut i = 0u64;
            let success = llvm::LLVMRustConstIntGetZExtValue(v, &mut i);
            success.then_some(i)
        })
    }

    fn const_to_opt_u128(&self, v: &'ll Value, sign_ext: bool) -> Option<u128> {
        try_as_const_integral(v).and_then(|v| unsafe {
            let (mut lo, mut hi) = (0u64, 0u64);
            let success = llvm::LLVMRustConstInt128Get(v, sign_ext, &mut hi, &mut lo);
            success.then_some(hi_lo_to_u128(lo, hi))
        })
    }

    fn scalar_to_backend(&self, cv: Scalar, layout: abi::Scalar, llty: &'ll Type) -> &'ll Value {
        let bitsize = if layout.is_bool() { 1 } else { layout.size(self).bits() };
        match cv {
            Scalar::Int(int) => {
                let data = int.to_bits(layout.size(self));
                let llval = self.const_uint_big(self.type_ix(bitsize), data);
                if matches!(layout.primitive(), Pointer(_)) {
                    unsafe { llvm::LLVMConstIntToPtr(llval, llty) }
                } else {
                    self.const_bitcast(llval, llty)
                }
            }
            Scalar::Ptr(ptr, _size) => {
                let (prov, offset) = ptr.into_parts();
                let (base_addr, base_addr_space) = match self.tcx.global_alloc(prov.alloc_id()) {
                    GlobalAlloc::Memory(alloc) => {
                        // For ZSTs directly codegen an aligned pointer.
                        // This avoids generating a zero-sized constant value and actually needing a
                        // real address at runtime.
                        if alloc.inner().len() == 0 {
                            assert_eq!(offset.bytes(), 0);
                            let llval = self.const_usize(alloc.inner().align.bytes());
                            return if matches!(layout.primitive(), Pointer(_)) {
                                unsafe { llvm::LLVMConstIntToPtr(llval, llty) }
                            } else {
                                self.const_bitcast(llval, llty)
                            };
                        } else {
                            let init = const_alloc_to_llvm(self, alloc, /*static*/ false);
                            let alloc = alloc.inner();
                            let value = match alloc.mutability {
                                Mutability::Mut => self.static_addr_of_mut(init, alloc.align, None),
                                _ => self.static_addr_of(init, alloc.align, None),
                            };
                            if !self.sess().fewer_names() && llvm::get_value_name(value).is_empty()
                            {
                                let hash = self.tcx.with_stable_hashing_context(|mut hcx| {
                                    let mut hasher = StableHasher::new();
                                    alloc.hash_stable(&mut hcx, &mut hasher);
                                    hasher.finish::<Hash128>()
                                });
                                llvm::set_value_name(
                                    value,
                                    format!("alloc_{hash:032x}").as_bytes(),
                                );
                            }
                            (value, AddressSpace::DATA)
                        }
                    }
                    GlobalAlloc::Function { instance, .. } => (
                        self.get_fn_addr(instance.polymorphize(self.tcx)),
                        self.data_layout().instruction_address_space,
                    ),
                    GlobalAlloc::VTable(ty, dyn_ty) => {
                        let alloc = self
                            .tcx
                            .global_alloc(self.tcx.vtable_allocation((ty, dyn_ty.principal())))
                            .unwrap_memory();
                        let init = const_alloc_to_llvm(self, alloc, /*static*/ false);
                        let value = self.static_addr_of(init, alloc.inner().align, None);
                        (value, AddressSpace::DATA)
                    }
                    GlobalAlloc::Static(def_id) => {
                        assert!(self.tcx.is_static(def_id));
                        assert!(!self.tcx.is_thread_local_static(def_id));
                        (self.get_static(def_id), AddressSpace::DATA)
                    }
                };
                let llval = unsafe {
                    llvm::LLVMConstInBoundsGEP2(
                        self.type_i8(),
                        self.const_bitcast(base_addr, self.type_ptr_ext(base_addr_space)),
                        &self.const_usize(offset.bytes()),
                        1,
                    )
                };
                if !matches!(layout.primitive(), Pointer(_)) {
                    unsafe { llvm::LLVMConstPtrToInt(llval, llty) }
                } else {
                    self.const_bitcast(llval, llty)
                }
            }
        }
    }

    fn const_data_from_alloc(&self, alloc: ConstAllocation<'tcx>) -> Self::Value {
        const_alloc_to_llvm(self, alloc, /*static*/ false)
    }

    fn const_ptr_byte_offset(&self, base_addr: Self::Value, offset: abi::Size) -> Self::Value {
        unsafe {
            llvm::LLVMConstInBoundsGEP2(
                self.type_i8(),
                base_addr,
                &self.const_usize(offset.bytes()),
                1,
            )
        }
    }
}

/// Get the [LLVM type][Type] of a [`Value`].
pub(crate) fn val_ty(v: &Value) -> &Type {
    unsafe { llvm::LLVMTypeOf(v) }
}

pub(crate) fn bytes_in_context<'ll>(llcx: &'ll llvm::Context, bytes: &[u8]) -> &'ll Value {
    unsafe {
        let ptr = bytes.as_ptr() as *const c_char;
        llvm::LLVMConstStringInContext2(llcx, ptr, bytes.len(), True)
    }
}

fn struct_in_context<'ll>(
    llcx: &'ll llvm::Context,
    elts: &[&'ll Value],
    packed: bool,
) -> &'ll Value {
    let len = c_uint::try_from(elts.len()).expect("LLVMConstStructInContext elements len overflow");
    unsafe { llvm::LLVMConstStructInContext(llcx, elts.as_ptr(), len, packed as Bool) }
}

#[inline]
fn hi_lo_to_u128(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

fn try_as_const_integral(v: &Value) -> Option<&ConstantInt> {
    unsafe { llvm::LLVMIsAConstantInt(v) }
}

pub(crate) fn get_dllimport<'tcx>(
    tcx: TyCtxt<'tcx>,
    id: DefId,
    name: &str,
) -> Option<&'tcx DllImport> {
    tcx.native_library(id)
        .and_then(|lib| lib.dll_imports.iter().find(|di| di.name.as_str() == name))
}