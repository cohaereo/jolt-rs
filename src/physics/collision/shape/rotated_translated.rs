use mint::{Quaternion, Vector3};

use crate::{HasShapeSettings, ShapeSettings};

#[repr(transparent)]
pub struct RotatedTranslatedShapeSettings(ShapeSettings);

impl RotatedTranslatedShapeSettings {
    pub fn new(
        shape_settings: &ShapeSettings,
        rotation: impl Into<Quaternion<f32>>,
        translation: impl Into<Vector3<f32>>,
    ) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_RotatedTranslatedShapeSettings_Create(
                    shape_settings.as_raw(),
                    rotation.into().as_ref().as_ptr(),
                    translation.into().as_ref().as_ptr(),
                ) as _,
            ))
        }
    }
}

impl HasShapeSettings for RotatedTranslatedShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for RotatedTranslatedShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
