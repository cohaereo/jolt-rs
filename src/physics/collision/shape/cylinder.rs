use crate::{shape::shape_settings::ShapeSettings, HasShapeSettings};

#[repr(transparent)]
pub struct CylinderShapeSettings(ShapeSettings);

impl CylinderShapeSettings {
    pub fn new(half_height: f32, radius: f32) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_CylinderShapeSettings_Create(half_height, radius) as _,
            ))
        }
    }

    pub fn half_height(&self) -> f32 {
        unsafe { jolt_sys::JPC_CylinderShapeSettings_GetHalfHeight(self.0.as_raw() as _) }
    }

    pub fn radius(&self) -> f32 {
        unsafe { jolt_sys::JPC_CylinderShapeSettings_GetRadius(self.0.as_raw() as _) }
    }
}

impl HasShapeSettings for CylinderShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for CylinderShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
