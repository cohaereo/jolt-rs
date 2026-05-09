use std::{marker::PhantomData, mem::MaybeUninit};

use mint::{Point3, Vector3};

use crate::{BodyId, Real, SubShapeId, Vec3Ext};

pub struct Ray {
    pub origin: Point3<Real>,
    pub direction: Vector3<f32>,
}

pub struct RayCastResult {
    pub body_id: BodyId,
    pub fraction: f32,
    pub sub_shape_id: SubShapeId,
}

pub struct NarrowPhaseQuery<'a>(*const jolt_sys::JPC_NarrowPhaseQuery, PhantomData<&'a ()>);

impl<'a> From<*const jolt_sys::JPC_NarrowPhaseQuery> for NarrowPhaseQuery<'a> {
    fn from(ptr: *const jolt_sys::JPC_NarrowPhaseQuery) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a> NarrowPhaseQuery<'a> {
    pub fn cast_ray(&self, ray: &Ray) -> Option<RayCastResult> {
        let ray = jolt_sys::JPC_RRayCast {
            origin: ray.origin.to_fixed_vec3(),
            direction: ray.direction.to_fixed_vec3(),
        };

        let mut result = jolt_sys::JPC_RayCastResult {
            body_id: u32::MAX,
            fraction: 1.0 + f32::EPSILON,
            sub_shape_id: u32::MAX,
        };

        let hit = unsafe {
            jolt_sys::JPC_NarrowPhaseQuery_CastRay(
                self.0,
                &raw const ray,
                &raw mut result,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        hit.then_some({
            RayCastResult {
                body_id: BodyId(result.body_id),
                fraction: result.fraction,
                sub_shape_id: SubShapeId(result.sub_shape_id),
            }
        })
    }
}
