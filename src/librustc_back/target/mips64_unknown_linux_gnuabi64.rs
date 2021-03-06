// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "mips64-unknown-linux-gnuabi64".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "64".to_string(),
        data_layout: "E-m:e-i8:8:32-i16:16:32-i64:64-n32:64-S128".to_string(),
        arch: "mips64".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        options: TargetOptions {
            // NOTE(mips64r2) matches C toolchain
            cpu: "mips64r2".to_string(),
            features: "+mips64r2".to_string(),
            max_atomic_width: Some(64),

            // see #36994
            exe_allocation_crate: "alloc_system".to_string(),

            ..super::linux_base::opts()
        },
    })
}
