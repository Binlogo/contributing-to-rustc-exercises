use crate::abi::Endian;
use crate::spec::{SanitizerSet, StackProbeType, Target, base};

pub(crate) fn target() -> Target {
    let mut base = base::linux_musl::opts();
    base.endian = Endian::Big;
    // z10 is the oldest CPU supported by LLVM
    base.cpu = "z10".into();
    // FIXME: The ABI implementation in abi/call/s390x.rs is for now hard-coded to assume the no-vector
    // ABI. Pass the -vector feature string to LLVM to respect this assumption.
    base.features = "-vector".into();
    base.max_atomic_width = Some(128);
    base.min_global_align = Some(16);
    base.static_position_independent_executables = true;
    base.stack_probes = StackProbeType::Inline;
    base.supported_sanitizers =
        SanitizerSet::ADDRESS | SanitizerSet::LEAK | SanitizerSet::MEMORY | SanitizerSet::THREAD;

    Target {
        llvm_target: "s390x-unknown-linux-musl".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("S390x Linux (kernel 3.2, musl 1.2.3)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout: "E-m:e-i1:8:16-i8:8:16-i64:64-f128:64-v128:64-a:8:16-n32:64".into(),
        arch: "s390x".into(),
        options: base,
    }
}