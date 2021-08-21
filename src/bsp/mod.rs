pub mod device_driver;

#[cfg(feature = "rpi3")]
pub mod rpi3;

#[cfg(feature = "rpi3")]
pub use rpi3 as device;
