mod r#box;
mod shape_settings;
mod sphere;

pub use r#box::*;
pub use shape_settings::*;
pub use sphere::*;

#[derive(Copy, Clone)]
pub struct ShapeRef(*mut jolt_sys::JPC_Shape);

impl ShapeRef {
    pub fn from_inner(inner: *mut jolt_sys::JPC_Shape) -> Self {
        Self(inner)
    }
}
