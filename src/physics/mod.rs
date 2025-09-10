pub mod body;
pub mod body_interface;
pub mod collision;
pub mod narrow_phase;
pub mod physics_system;

pub use body::*;
pub use body_interface::*;
pub use collision::*;
pub use physics_system::*;

/// Type alias for the real number type used in Jolt Physics. This is typically used in positions.
///
/// Currently only `f32`.
pub type Real = jolt_sys::JPC_Real;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Activation {
    Activate = jolt_sys::JPC_Activation_JPC_ACTIVATION_ACTIVATE as _,
    DontActivate = jolt_sys::JPC_Activation_JPC_ACTIVATION_DONT_ACTIVATE as _,
}
