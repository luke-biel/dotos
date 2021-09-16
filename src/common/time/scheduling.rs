pub trait SchedulingManager {
    fn register_handler(
        &self,
        handler: &'static (dyn TickCallbackHandler + Sync),
    ) -> Result<(), &'static str>;
}

pub trait TickCallbackHandler {
    fn handle(&self);
}
