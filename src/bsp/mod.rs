pub mod device_driver;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "rpi1")] {
        mod rpi;
        pub use rpi::*;
    } else {
        compile_error!("Unsupported board type");
    }
}
