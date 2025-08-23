use mint::Vector3;

use crate::{HasShapeSettings, ShapeSettings};

#[repr(transparent)]
pub struct ScaledShapeSettings(ShapeSettings);

impl ScaledShapeSettings {
    pub fn new(shape_settings: &ShapeSettings, scale: impl Into<Vector3<f32>>) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_ScaledShapeSettings_Create(
                    shape_settings.as_raw(),
                    scale.into().as_ref().as_ptr(),
                ) as _,
            ))
        }
    }
}

impl HasShapeSettings for ScaledShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for ScaledShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
