use std::ffi::c_void;

use mint::{Quaternion, Vector3};

use crate::{HasShapeSettings, Shape, ShapeSettings};

#[repr(transparent)]
pub struct ConvexHullShapeSettings(ShapeSettings);

impl ConvexHullShapeSettings {
    pub fn new(vertices: &[Vector3<f32>]) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_ConvexHullShapeSettings_Create(
                    vertices.as_ptr() as *const c_void,
                    vertices.len() as u32,
                    std::mem::size_of::<Vector3<f32>>() as u32,
                ) as _,
            ))
        }
    }
}

impl HasShapeSettings for ConvexHullShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for ConvexHullShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
