use crate::{
    arch::arch_impl::cpu::{exception::ExceptionContext, instructions::eret},
    common::{
        exception::asynchronous::{IRQContext, IRQManager},
        statics,
    },
};

unsafe fn default_handler(e: &mut ExceptionContext) {
    let far_el1: u64;
    asm!("mrs {}, far_el1", out(reg) far_el1, options(nostack, nomem));
    panic!("CPU Exception\nfar_el1: {:#018x}\n{}", far_el1, e)
}

#[no_mangle]
unsafe extern "C" fn current_el0_sync(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_el0_irq(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_el0_serror(_e: &mut ExceptionContext) {
    panic!("unsupported exception")
}

#[no_mangle]
unsafe extern "C" fn current_elx_sync(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(_e: &mut ExceptionContext) {
    let token = IRQContext::new();
    statics::INTERRUPT_CONTROLLER.handle_pending(token)
}

#[no_mangle]
unsafe extern "C" fn current_elx_serror(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_sync(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_sync(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext) {
    default_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext) {
    default_handler(e)
}
