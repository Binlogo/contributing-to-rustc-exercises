use crate::spec::{Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions, base};

pub(crate) fn target() -> Target {
    let mut base = base::linux_musl::opts();
    base.cpu = "ppc64le".into();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]);
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: "powerpc64le-unknown-linux-musl".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("64-bit PowerPC Linux with musl 1.2.3, Little Endian".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: None, // ?
        },
        pointer_width: 64,
        data_layout: "e-m:e-Fn32-i64:64-n32:64-S128-v256:256:256-v512:512:512".into(),
        arch: "powerpc64".into(),
        options: TargetOptions { mcount: "_mcount".into(), ..base },
    }
}