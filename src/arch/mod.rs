#![allow(dead_code)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "arch-arm")] {
        mod arm;
        pub use arm::*;

        pub type Int = i32;
        pub type UInt = u32;
        pub type IntPtr = u32;
    } else {
        compile_error!("Unsupported architecture type");
    }
}
