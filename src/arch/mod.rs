#![allow(dead_code)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "aarch32")] {
        mod aarch32;
        pub use aarch32::*;

        pub type Int = i32;
        pub type UInt = u32;
        pub type IntPtr = u32;
    } else {
        compile_error!("Unsupported architecture type");
    }
}
