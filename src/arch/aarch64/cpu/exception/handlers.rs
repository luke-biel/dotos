use crate::{
    arch::arch_impl::cpu::{
        exception::ExceptionContext,
        registers::{esr_el1::EsrEl1, far_el1::FarEl1},
    },
    common::{
        exception::asynchronous::{IRQContext, IRQManager},
        statics,
    },
};

unsafe fn default_handler(kind: &'static str, e: &mut ExceptionContext) {
    let far_el1 = FarEl1::new().get();
    let esr_el1 = EsrEl1::fetch();
    panic!(
        "CPU Exception `{}`\n{}FAR_EL1:  {:#018x}\nESR_EL1:\n{}",
        kind, e, far_el1, esr_el1,
    )
}

#[no_mangle]
unsafe extern "C" fn current_el1t_sync(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_el1t_irq(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_el1t_serror(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_el1h_sync(e: &mut ExceptionContext) {
    default_handler("current_el1h_sync", e)
}

#[no_mangle]
unsafe extern "C" fn current_el1h_irq(_e: &mut ExceptionContext) {
    let token = IRQContext::new();
    statics::INTERRUPT_CONTROLLER.handle_pending(token)
}

#[no_mangle]
unsafe extern "C" fn current_el1h_serror(e: &mut ExceptionContext) {
    default_handler("current_elx_serror", e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_sync(e: &mut ExceptionContext) {
    default_handler("lower_aarch64_sync", e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(_e: &mut ExceptionContext) {
    let token = IRQContext::new();
    statics::INTERRUPT_CONTROLLER.handle_pending(token)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext) {
    default_handler("lower_aarch64_serror", e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_sync(e: &mut ExceptionContext) {
    default_handler("lower_aarch32_sync", e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext) {
    default_handler("lower_aarch32_irq", e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext) {
    default_handler("lower_aarch32_serror", e)
}
