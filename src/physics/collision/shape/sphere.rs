use crate::{ShapeRef, ShapeSettings};
use jolt_sys::JPC_ShapeSettings;

#[repr(transparent)]
pub struct SphereShapeSettings(*mut jolt_sys::JPC_SphereShapeSettings);

impl SphereShapeSettings {
    pub fn create(radius: f32) -> Self {
        unsafe { Self(jolt_sys::JPC_SphereShapeSettings_Create(radius)) }
    }
}

impl ShapeSettings for SphereShapeSettings {
    fn as_shape_settings(&self) -> *const JPC_ShapeSettings {
        self.0 as *mut JPC_ShapeSettings
    }
}

/// Emulates `JPH::SphereShape`
pub struct SphereShape;

impl SphereShape {
    pub fn create(radius: f32) -> ShapeRef {
        let shape_settings = SphereShapeSettings::create(radius);

        shape_settings.create_shape().unwrap()
    }
}
