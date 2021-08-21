use core::time::Duration;

pub trait TimeManager {
    fn resolution(&self) -> Duration;
    fn uptime(&self) -> Duration;
    fn sleep(&self, duration: Duration);
}
