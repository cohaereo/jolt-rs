use mint::{Quaternion, Vector3};

use crate::{HasShapeSettings, ShapeSettings};

#[repr(transparent)]
pub struct StaticCompoundShapeSettings(ShapeSettings);

impl StaticCompoundShapeSettings {
    pub fn new() -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_StaticCompoundShapeSettings_Create() as _,
            ))
        }
    }

    pub fn add_shape(
        &mut self,
        shape_settings: &impl AsRef<ShapeSettings>,
        position: impl Into<Vector3<f32>>,
        rotation: impl Into<Quaternion<f32>>,
        user_data: u32,
    ) {
        unsafe {
            jolt_sys::JPC_CompoundShapeSettings_AddShape(
                self.0.as_raw() as *mut jolt_sys::JPC_CompoundShapeSettings,
                position.into().as_ref().as_ptr(),
                rotation.into().as_ref().as_ptr(),
                shape_settings.as_ref().as_raw() as _,
                user_data,
            );
        }
    }
}

impl HasShapeSettings for StaticCompoundShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for StaticCompoundShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
