use crate::spec::base::apple::{Arch, TargetAbi, base};
use crate::spec::{FramePointer, SanitizerSet, Target, TargetOptions};

pub(crate) fn target() -> Target {
    let (opts, llvm_target, arch) = base("visionos", Arch::Arm64, TargetAbi::Normal);
    Target {
        llvm_target,
        metadata: crate::spec::TargetMetadata {
            description: Some("ARM64 Apple visionOS".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128-Fn32".into(),
        arch,
        options: TargetOptions {
            features: "+neon,+fp-armv8,+apple-a16".into(),
            max_atomic_width: Some(128),
            frame_pointer: FramePointer::NonLeaf,
            supported_sanitizers: SanitizerSet::ADDRESS | SanitizerSet::THREAD,
            ..opts
        },
    }
}