use std::{marker::PhantomData, mem::MaybeUninit};

use mint::{Point3, Vector3};

use crate::Vec3Ext;

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

pub struct NarrowPhaseQuery<'a>(*const jolt_sys::JPC_NarrowPhaseQuery, PhantomData<&'a ()>);

impl<'a> From<*const jolt_sys::JPC_NarrowPhaseQuery> for NarrowPhaseQuery<'a> {
    fn from(ptr: *const jolt_sys::JPC_NarrowPhaseQuery) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a> NarrowPhaseQuery<'a> {
    pub fn cast_ray(&self, ray: &Ray) -> Option<jolt_sys::JPC_RayCastResult> {
        let ray = jolt_sys::JPC_RRayCast {
            origin: ray.origin.to_fixed_vec3(),
            direction: ray.direction.to_fixed_vec3(),
        };

        let mut result = MaybeUninit::<jolt_sys::JPC_RayCastResult>::zeroed();

        let hit = unsafe {
            jolt_sys::JPC_NarrowPhaseQuery_CastRay(
                self.0,
                &raw const ray,
                result.as_mut_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        hit.then_some(unsafe { result.assume_init() })
    }
}
