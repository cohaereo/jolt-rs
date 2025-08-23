use std::ffi::c_void;

use mint::{Quaternion, Vector3};

use crate::{HasShapeSettings, Shape, ShapeSettings};

#[repr(transparent)]
pub struct MeshShapeSettings(ShapeSettings);

impl MeshShapeSettings {
    pub fn new(vertices: &[Vector3<f32>], indices: &[u32]) -> Self {
        unsafe {
            Self(ShapeSettings::from_raw(
                jolt_sys::JPC_MeshShapeSettings_Create(
                    vertices.as_ptr() as *const c_void,
                    vertices.len() as u32,
                    std::mem::size_of::<Vector3<f32>>() as u32,
                    indices.as_ptr(),
                    indices.len() as u32,
                ) as _,
            ))
        }
    }
}

impl HasShapeSettings for MeshShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings {
        &self.0
    }
}

impl AsRef<ShapeSettings> for MeshShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self.as_shape_settings()
    }
}
