use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "arch-arm")] {
        mod arm;
        pub use arm::*;
    } else {
        compile_error!("Unsupported architecture type");
    }
}
