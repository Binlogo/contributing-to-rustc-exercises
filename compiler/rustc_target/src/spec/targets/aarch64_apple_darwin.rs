use crate::spec::base::apple::{Arch, TargetAbi, base};
use crate::spec::{FramePointer, SanitizerSet, Target, TargetOptions};

pub(crate) fn target() -> Target {
    let (opts, llvm_target, arch) = base("macos", Arch::Arm64, TargetAbi::Normal);
    Target {
        llvm_target,
        metadata: crate::spec::TargetMetadata {
            description: Some("ARM64 Apple macOS (11.0+, Big Sur+)".into()),
            tier: Some(1),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128-Fn32".into(),
        arch,
        options: TargetOptions {
            mcount: "\u{1}mcount".into(),
            frame_pointer: FramePointer::NonLeaf,
            cpu: "apple-m1".into(),
            max_atomic_width: Some(128),
            // FIXME: The leak sanitizer currently fails the tests, see #88132.
            supported_sanitizers: SanitizerSet::ADDRESS | SanitizerSet::CFI | SanitizerSet::THREAD,
            ..opts
        },
    }
}