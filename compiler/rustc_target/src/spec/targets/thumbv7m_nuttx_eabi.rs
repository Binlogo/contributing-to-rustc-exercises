// Targets the Cortex-M3 processor (ARMv7-M)

use crate::spec::{Target, TargetOptions, base, cvs};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "thumbv7m-none-eabi".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            families: cvs!["unix"],
            os: "nuttx".into(),
            abi: "eabi".into(),
            max_atomic_width: Some(32),
            ..base::thumb::opts()
        },
    }
}