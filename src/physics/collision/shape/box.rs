use crate::shape::shape_settings::ShapeSettings;
use crate::ShapeRef;
use jolt_sys::JPC_ShapeSettings;
use mint::Vector3;

#[repr(transparent)]
pub struct BoxShapeSettings(*mut jolt_sys::JPC_BoxShapeSettings);

impl BoxShapeSettings {
    pub fn create<V>(half_extents: V) -> Self
    where
        V: Into<Vector3<f32>>,
    {
        unsafe {
            Self(jolt_sys::JPC_BoxShapeSettings_Create(
                half_extents.into().as_ref().as_ptr(),
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
    pub fn create<V>(half_extents: V) -> ShapeRef
    where
        V: Into<Vector3<f32>>,
    {
        let shape_settings = BoxShapeSettings::create(half_extents);

        shape_settings.create_shape().unwrap()
    }
}
