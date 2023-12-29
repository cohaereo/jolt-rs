use jolt_sys::{
    JPC_EMotionQuality_JPC_MOTION_QUALITY_DISCRETE,
    JPC_EMotionQuality_JPC_MOTION_QUALITY_LINEAR_CAST, JPC_EMotionType_JPC_MOTION_TYPE_DYNAMIC,
    JPC_EMotionType_JPC_MOTION_TYPE_KINEMATIC, JPC_EMotionType_JPC_MOTION_TYPE_STATIC,
};

mod body_activation_listener;
mod contact_listener;
mod creation_settings;

pub use body_activation_listener::*;
pub use contact_listener::*;
pub use creation_settings::*;

pub type BodyId = jolt_sys::JPC_BodyID;
pub type Body = jolt_sys::JPC_Body;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MotionType {
    Static = JPC_EMotionType_JPC_MOTION_TYPE_STATIC as _,
    Kinematic = JPC_EMotionType_JPC_MOTION_TYPE_KINEMATIC as _,
    Dynamic = JPC_EMotionType_JPC_MOTION_TYPE_DYNAMIC as _,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MotionQuality {
    Discrete = JPC_EMotionQuality_JPC_MOTION_QUALITY_DISCRETE as _,
    LinearCast = JPC_EMotionQuality_JPC_MOTION_QUALITY_LINEAR_CAST as _,
}
