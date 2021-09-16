use core::time::Duration;

pub trait ClockManager {
    fn resolution(&self) -> Duration;
    fn uptime(&self) -> Duration;
    fn sleep(&self, duration: Duration);
}
