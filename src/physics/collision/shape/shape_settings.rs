use std::fmt::Debug;

use crate::Shape;
use jolt_sys::JPC_ShapeSettings;

pub struct ShapeSettings(*mut JPC_ShapeSettings);

impl ShapeSettings {
    pub fn as_raw(&self) -> *mut JPC_ShapeSettings {
        self.0
    }

    pub fn from_raw(raw: *mut JPC_ShapeSettings) -> Self {
        assert!(!raw.is_null());
        Self(raw)
    }

    pub fn create_shape(&self) -> Result<Shape, String> {
        unsafe {
            let shape = jolt_sys::JPC_ShapeSettings_CreateShape(self.as_raw());
            if shape.is_null() {
                let error_c = jolt_sys::JPC_ShapeSettings_GetError(self.as_raw());
                let error = std::ffi::CStr::from_ptr(error_c)
                    .to_string_lossy()
                    .into_owned();
                Err(error)
            } else {
                Ok(Shape::from_raw(shape))
            }
        }
    }
}

impl Clone for ShapeSettings {
    fn clone(&self) -> Self {
        assert!(!self.0.is_null());
        unsafe {
            jolt_sys::JPC_ShapeSettings_AddRef(self.as_raw());
        }
        Self(self.0)
    }
}

impl Drop for ShapeSettings {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe {
            jolt_sys::JPC_ShapeSettings_Release(self.as_raw());
        }
    }
}

impl Debug for ShapeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShapeSettings")
            .field("raw", &self.0)
            .finish()
    }
}

impl AsRef<ShapeSettings> for ShapeSettings {
    fn as_ref(&self) -> &ShapeSettings {
        self
    }
}

unsafe impl Send for ShapeSettings {}
unsafe impl Sync for ShapeSettings {}

pub trait HasShapeSettings {
    fn as_shape_settings(&self) -> &ShapeSettings;

    fn to_shape_settings(&self) -> ShapeSettings {
        self.as_shape_settings().clone()
    }

    fn create_shape(&self) -> Result<Shape, String> {
        self.as_shape_settings().create_shape()
    }
}

impl<S: HasShapeSettings> From<S> for ShapeSettings {
    fn from(value: S) -> Self {
        value.to_shape_settings()
    }
}
