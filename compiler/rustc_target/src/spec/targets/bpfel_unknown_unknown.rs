use crate::spec::base::sbf_base;
use crate::spec::{Arch, Target, TargetMetadata};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "bpfel".into(),
        pointer_width: 64,
        arch: Arch::Bpf,
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        // FIXME(nagisa): is this right??
        options: sbf_base::opts("v0"),
        metadata: TargetMetadata { description: None, tier: None, host_tools: None, std: None },
    }
}
