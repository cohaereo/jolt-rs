mod r#box;
mod capsule;
mod compound;
mod convex_hull;
mod cylinder;
mod mesh;
mod rotated_translated;
mod scaled;
mod shape_settings;
mod sphere;

use std::ptr::NonNull;

pub use capsule::*;
pub use compound::*;
pub use convex_hull::*;
pub use cylinder::*;
use jolt_sys::JPC_MassProperties;
pub use mesh::*;
use mint::Vector3;
pub use r#box::*;
pub use rotated_translated::*;
pub use scaled::*;
pub use shape_settings::*;
pub use sphere::*;

pub type SubShapeIDPair = jolt_sys::JPC_SubShapeIDPair;

pub struct Shape(NonNull<jolt_sys::JPC_Shape>);

impl Shape {
    pub fn from_raw(inner: *mut jolt_sys::JPC_Shape) -> Self {
        Self(NonNull::new(inner).expect("Shape pointer is NULL"))
    }

    pub fn center_of_mass(&self) -> Vector3<f32> {
        let mut position = [0.0; 3];
        unsafe {
            jolt_sys::JPC_Shape_GetCenterOfMass(self.0.as_ptr(), position.as_mut_ptr());
        }
        Vector3::from(position)
    }

    pub fn mass_properties(&self) -> JPC_MassProperties {
        let mut properties: JPC_MassProperties = unsafe { std::mem::zeroed() };
        unsafe {
            jolt_sys::JPC_Shape_GetMassProperties(self.0.as_ptr(), &mut properties);
        }
        properties
    }

    pub fn shape_type(&self) -> ShapeType {
        unsafe {
            std::mem::transmute::<jolt_sys::JPC_ShapeType, ShapeType>(jolt_sys::JPC_Shape_GetType(
                self.0.as_ptr(),
            ))
        }
    }

    pub fn shape_subtype(&self) -> ShapeSubType {
        unsafe {
            std::mem::transmute::<jolt_sys::JPC_ShapeSubType, ShapeSubType>(
                jolt_sys::JPC_Shape_GetSubType(self.0.as_ptr()),
            )
        }
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        unsafe {
            jolt_sys::JPC_Shape_AddRef(self.0.as_ptr());
        }
        Self(self.0)
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe {
            jolt_sys::JPC_Shape_Release(self.0.as_ptr());
        }
    }
}

unsafe impl Send for Shape {}
unsafe impl Sync for Shape {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Convex = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_CONVEX as u8,
    Compound = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_COMPOUND as u8,
    Decorated = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_DECORATED as u8,
    Mesh = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_MESH as u8,
    HeightField = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_HEIGHT_FIELD as u8,
    User1 = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_USER1 as u8,
    User2 = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_USER2 as u8,
    User3 = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_USER3 as u8,
    User4 = jolt_sys::JPC_EShapeType_JPC_SHAPE_TYPE_USER4 as u8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeSubType {
    Sphere = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_SPHERE as u8,
    Box = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_BOX as u8,
    Triangle = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_TRIANGLE as u8,
    Capsule = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_CAPSULE as u8,
    TaperedCapsule = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_TAPERED_CAPSULE as u8,
    Cylinder = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_CYLINDER as u8,
    ConvexHull = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_CONVEX_HULL as u8,
    StaticCompound = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_STATIC_COMPOUND as u8,
    MutableCompound = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_MUTABLE_COMPOUND as u8,
    RotatedTranslated = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_ROTATED_TRANSLATED as u8,
    Scaled = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_SCALED as u8,
    OffsetCenterOfMass = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_OFFSET_CENTER_OF_MASS as u8,
    Mesh = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_MESH as u8,
    HeightField = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_HEIGHT_FIELD as u8,
    User1 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER1 as u8,
    User2 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER2 as u8,
    User3 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER3 as u8,
    User4 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER4 as u8,
    User5 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER5 as u8,
    User6 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER6 as u8,
    User7 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER7 as u8,
    User8 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER8 as u8,
    UserConvex1 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX1 as u8,
    UserConvex2 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX2 as u8,
    UserConvex3 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX3 as u8,
    UserConvex4 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX4 as u8,
    UserConvex5 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX5 as u8,
    UserConvex6 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX6 as u8,
    UserConvex7 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX7 as u8,
    UserConvex8 = jolt_sys::JPC_EShapeSubType_JPC_SHAPE_SUB_TYPE_USER_CONVEX8 as u8,
}
