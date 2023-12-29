use crate::shape::shape_settings::ShapeSettings;
use crate::ShapeRef;
use glam::Vec3;
use jolt_sys::JPC_ShapeSettings;

#[repr(transparent)]
pub struct BoxShapeSettings(*mut jolt_sys::JPC_BoxShapeSettings);

impl BoxShapeSettings {
    pub fn create(half_extents: Vec3) -> Self {
        unsafe {
            Self(jolt_sys::JPC_BoxShapeSettings_Create(
                half_extents.as_ref().as_ptr(),
            ))
        }
    }
}

impl ShapeSettings for BoxShapeSettings {
    fn as_shape_settings(&self) -> *const JPC_ShapeSettings {
        self.0 as *mut JPC_ShapeSettings
    }
}

/// Emulates `JPH::BoxShape`
pub struct BoxShape;

impl BoxShape {
    pub fn create(half_extents: Vec3) -> ShapeRef {
        let shape_settings = BoxShapeSettings::create(half_extents);

        shape_settings.create_shape().unwrap()
    }
}
