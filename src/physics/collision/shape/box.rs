use crate::{shape::shape_settings::ShapeSettings, HasShapeSettings, Shape};
use mint::Vector3;

#[repr(transparent)]
pub struct BoxShapeSettings(ShapeSettings);

impl BoxShapeSettings {
    pub fn new<V>(half_extents: V) -> Self
    where
        V: Into<Vector3<f32>>,
    {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_BoxShapeSettings_Create(half_extents.into().as_ref().as_ptr()) as _,
            ))
        }
    }

    pub fn half_extent(&self) -> Vector3<f32> {
        unsafe {
            let mut half_extent = [0.0; 3];
            jolt_sys::JPC_BoxShapeSettings_GetHalfExtent(
                self.0.as_raw() as _,
                half_extent.as_mut_ptr(),
            );

            half_extent.into()
        }
    }

    pub fn convex_radius(&self) -> f32 {
        unsafe { jolt_sys::JPC_BoxShapeSettings_GetConvexRadius(self.0.as_raw() as _) }
    }
}

impl HasShapeSettings for BoxShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for BoxShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
