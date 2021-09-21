use core::sync::atomic::{AtomicU8, Ordering};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, PartialEq)]
pub enum KernelState {
    Init,
    SingleCoreRun,
}

pub struct KernelInitManager {
    state: AtomicU8,
}

impl KernelInitManager {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::default(),
        }
    }

    pub(crate) fn is_init(&self) -> bool {
        let state = self.state.load(Ordering::Acquire);
        KernelState::from_u8(state).expect("KernelState::from_u8") == KernelState::Init
    }

    pub fn transition(&self, from: KernelState, to: KernelState) {
        if self
            .state
            .compare_exchange(
                from.to_u8().expect("from to_u8"),
                to.to_u8().expect("to to_u8"),
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            panic!("Failed to transition kernel state")
        }
    }
}
