use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum SysCall {
    Exit = 0,
    Write = 1,
}
