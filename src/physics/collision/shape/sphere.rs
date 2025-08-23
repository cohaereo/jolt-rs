use crate::{HasShapeSettings, Shape, ShapeSettings};

#[repr(transparent)]
pub struct SphereShapeSettings(ShapeSettings);

impl SphereShapeSettings {
    pub fn new(radius: f32) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_SphereShapeSettings_Create(radius) as _,
            ))
        }
    }
}

impl HasShapeSettings for SphereShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for SphereShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
