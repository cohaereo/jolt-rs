use crate::{
    narrow_phase::NarrowPhaseQuery, BodyActivationListener, BodyActivationListenerWrapper,
    BodyInterface, BroadPhaseLayerInterface, BroadPhaseLayerInterfaceWrapper, ContactListener,
    ContactListenerWrapper, JobSystem, ObjectLayerPairFilter, ObjectLayerPairFilterWrapper,
    ObjectVsBroadPhaseLayerFilter, ObjectVsBroadPhaseLayerFilterWrapper, TempAllocator,
};
use jolt_sys::JPC_EPhysicsUpdateError;
use mint::Vector3;
use std::ffi::c_void;

pub struct PhysicsSystem {
    inner: *mut jolt_sys::JPC_PhysicsSystem,
}

impl PhysicsSystem {
    pub fn create(
        max_bodies: u32,
        num_body_mutexes: u32,
        max_body_pairs: u32,
        max_contact_constraints: u32,
        broad_phase_layer: Box<dyn BroadPhaseLayerInterface>,
        object_vs_broad_phase_layer_filter: Box<dyn ObjectVsBroadPhaseLayerFilter>,
        object_layer_pair_filter: Box<dyn ObjectLayerPairFilter>,
    ) -> Self {
        let broad_phase_layer_wrapper =
            Box::new(BroadPhaseLayerInterfaceWrapper::new(broad_phase_layer));
        let object_vs_broad_phase_layer_filter_wrapper = Box::new(
            ObjectVsBroadPhaseLayerFilterWrapper::new(object_vs_broad_phase_layer_filter),
        );
        let object_layer_pair_filter_wrapper =
            Box::new(ObjectLayerPairFilterWrapper::new(object_layer_pair_filter));

        unsafe {
            PhysicsSystem {
                inner: jolt_sys::JPC_PhysicsSystem_Create(
                    max_bodies,
                    num_body_mutexes,
                    max_body_pairs,
                    max_contact_constraints,
                    (Box::into_raw(broad_phase_layer_wrapper)
                        as *const BroadPhaseLayerInterfaceWrapper)
                        as *const c_void,
                    (Box::into_raw(object_vs_broad_phase_layer_filter_wrapper)
                        as *const ObjectVsBroadPhaseLayerFilterWrapper)
                        as *const c_void,
                    (Box::into_raw(object_layer_pair_filter_wrapper)
                        as *const ObjectLayerPairFilterWrapper)
                        as *const c_void,
                ),
            }
        }
    }
}

impl PhysicsSystem {
    pub fn set_body_activation_listener(
        &self,
        body_activation_listener: Box<dyn BodyActivationListener>,
    ) {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_SetBodyActivationListener(
                self.inner,
                Box::into_raw(Box::new(BodyActivationListenerWrapper::new(
                    body_activation_listener,
                ))) as *const BodyActivationListenerWrapper as *mut c_void,
            );
        }
    }

    pub fn set_contact_listener(&self, contact_listener: Box<dyn ContactListener>) {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_SetContactListener(
                self.inner,
                Box::into_raw(Box::new(ContactListenerWrapper::new(contact_listener)))
                    as *const ContactListenerWrapper as *mut c_void,
            );
        }
    }

    pub fn num_bodies(&self) -> u32 {
        unsafe { jolt_sys::JPC_PhysicsSystem_GetNumBodies(self.inner) }
    }

    pub fn num_active_bodies(&self) -> u32 {
        unsafe { jolt_sys::JPC_PhysicsSystem_GetNumActiveBodies(self.inner) }
    }

    pub fn max_bodies(&self) -> u32 {
        unsafe { jolt_sys::JPC_PhysicsSystem_GetMaxBodies(self.inner) }
    }

    pub fn gravity(&self) -> Vector3<f32> {
        unsafe {
            let mut result = Vector3::from([0.; 3]);
            jolt_sys::JPC_PhysicsSystem_GetGravity(self.inner, result.as_mut().as_mut_ptr());
            result
        }
    }

    pub fn set_gravity<V>(&self, gravity: V)
    where
        V: Into<Vector3<f32>>,
    {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_SetGravity(self.inner, gravity.into().as_ref().as_ptr());
        }
    }

    pub fn body_interface<'a>(&'a self) -> BodyInterface<'a> {
        BodyInterface::from(unsafe { jolt_sys::JPC_PhysicsSystem_GetBodyInterface(self.inner) })
    }

    pub fn optimize_broad_phase(&self) {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_OptimizeBroadPhase(self.inner);
        }
    }

    // TODO: AddStepListener, RemoveStepListener, AddConstraint, RemoveConstraint

    pub fn update(
        &self,
        delta_time: f32,
        collision_steps: i32,
        integration_sub_steps: i32,
        temp_allocator: &mut TempAllocator,
        job_system: &mut JobSystem,
    ) -> JPC_EPhysicsUpdateError {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_Update(
                self.inner,
                delta_time,
                collision_steps,
                integration_sub_steps,
                temp_allocator.as_ptr(),
                job_system.as_ptr(),
            ) as _
        }
    }

    // TODO: GetBodyLockInterface, GetBodyLockInterfaceNoLock

    pub fn narrow_phase_query<'a>(&'a self) -> NarrowPhaseQuery<'a> {
        NarrowPhaseQuery::from(unsafe {
            jolt_sys::JPC_PhysicsSystem_GetNarrowPhaseQuery(self.inner)
        })
    }
}

impl Drop for PhysicsSystem {
    fn drop(&mut self) {
        unsafe {
            jolt_sys::JPC_PhysicsSystem_Destroy(self.inner);
        }
    }
}
