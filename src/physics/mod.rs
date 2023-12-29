pub mod body;
pub mod body_interface;
pub mod collision;
pub mod physics_system;

pub use body::*;
pub use body_interface::*;
pub use collision::*;
use jolt_sys::{
    JPC_Activation_JPC_ACTIVATION_ACTIVATE, JPC_Activation_JPC_ACTIVATION_DONT_ACTIVATE,
};
pub use physics_system::*;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Activation {
    Activate = JPC_Activation_JPC_ACTIVATION_ACTIVATE as _,
    DontActivate = JPC_Activation_JPC_ACTIVATION_DONT_ACTIVATE as _,
}
