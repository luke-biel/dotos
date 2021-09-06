use core::marker::PhantomData;

pub trait IRQHandler {
    fn handle(&self) -> Result<(), &'static str>;
}

pub trait IRQManager {
    type IRQNumberT;

    fn register_handler(
        &self,
        irq: Self::IRQNumberT,
        descriptor: IRQDescriptor,
    ) -> Result<(), &'static str>;
    fn enable(&self, irq: Self::IRQNumberT);
    fn handle_pending<'ctx>(&'ctx self, token: IRQContext<'ctx>);
}

pub struct IRQContext<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
}

#[derive(Copy, Clone)]
pub struct IRQDescriptor {
    pub name: &'static str,
    pub handler: &'static (dyn IRQHandler + Sync),
}

impl<'ctx> IRQContext<'ctx> {
    pub unsafe fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
