use crate::{
    core::Vec3Ext, Activation, Body, BodyCreationSettings, BodyId, MotionType, ObjectLayer,
};
use mint::{Point3, Quaternion, Vector3};
use std::{marker::PhantomData, mem::transmute};

pub struct BodyInterface<'a>(*mut jolt_sys::JPC_BodyInterface, PhantomData<&'a ()>);

impl<'a> From<*mut jolt_sys::JPC_BodyInterface> for BodyInterface<'a> {
    fn from(ptr: *mut jolt_sys::JPC_BodyInterface) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a> BodyInterface<'a> {
    pub fn create_body(&self, body_settings: &BodyCreationSettings) -> Option<*mut Body> {
        unsafe {
            let body_id = jolt_sys::JPC_BodyInterface_CreateBody(self.0, &body_settings.to_jpc());
            if body_id.is_null() {
                None
            } else {
                Some(body_id)
            }
        }
    }

    pub fn create_body_with_id(
        &self,
        body_id: BodyId,
        body_settings: &BodyCreationSettings,
    ) -> Option<*mut Body> {
        unsafe {
            let body_id = jolt_sys::JPC_BodyInterface_CreateBodyWithID(
                self.0,
                body_id,
                &body_settings.to_jpc(),
            );
            if body_id.is_null() {
                None
            } else {
                Some(body_id)
            }
        }
    }

    pub fn destroy_body(&self, body_id: BodyId) {
        unsafe {
            jolt_sys::JPC_BodyInterface_DestroyBody(self.0, body_id);
        }
    }

    pub fn add_body(&self, body_id: BodyId, activation: Activation) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddBody(self.0, body_id, activation as _);
        }
    }

    pub fn remove_body(&self, body_id: BodyId) {
        unsafe {
            jolt_sys::JPC_BodyInterface_RemoveBody(self.0, body_id);
        }
    }

    pub fn create_and_add_body(
        &self,
        body_settings: &BodyCreationSettings,
        activation: Activation,
    ) -> BodyId {
        unsafe {
            jolt_sys::JPC_BodyInterface_CreateAndAddBody(
                self.0,
                &body_settings.to_jpc(),
                activation as _,
            )
        }
    }

    pub fn is_added(&self, body_id: BodyId) -> bool {
        unsafe { jolt_sys::JPC_BodyInterface_IsAdded(self.0, body_id) }
    }

    pub fn set_linear_and_angular_velocity(
        &self,
        body_id: BodyId,
        linear: impl Into<Vector3<f32>>,
        angular: impl Into<Vector3<f32>>,
    ) {
        let linear = linear.into().to_fixed_vec3();
        let angular = angular.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_SetLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_ptr(),
                angular.as_ptr(),
            );
        }
    }

    pub fn linear_and_angular_velocity(&self, body_id: BodyId) -> (Vector3<f32>, Vector3<f32>) {
        unsafe {
            let mut linear = [0.; 3];
            let mut angular = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_mut_ptr(),
                angular.as_mut_ptr(),
            );

            (Vector3::from(linear), Vector3::from(angular))
        }
    }

    pub fn set_linear_velocity(&self, body_id: BodyId, velocity: impl Into<Vector3<f32>>) {
        let velocity = velocity.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_SetLinearVelocity(self.0, body_id, velocity.as_ptr());
        }
    }

    pub fn linear_velocity(&self, body_id: BodyId) -> Vector3<f32> {
        unsafe {
            let mut result = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetLinearVelocity(self.0, body_id, result.as_mut_ptr());

            Vector3::from(result)
        }
    }

    pub fn add_linear_velocity(&self, body_id: BodyId, velocity: impl Into<Vector3<f32>>) {
        let velocity = velocity.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddLinearVelocity(self.0, body_id, velocity.as_ptr());
        }
    }

    pub fn add_linear_and_angular_velocity(
        &self,
        body_id: BodyId,
        linear: impl Into<Vector3<f32>>,
        angular: impl Into<Vector3<f32>>,
    ) {
        let linear = linear.to_fixed_vec3();
        let angular = angular.to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_ptr(),
                angular.as_ptr(),
            );
        }
    }

    pub fn set_angular_velocity(&self, body_id: BodyId, velocity: impl Into<Vector3<f32>>) {
        let v = velocity.to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_SetAngularVelocity(self.0, body_id, v.as_ptr());
        }
    }

    pub fn angular_velocity(&self, body_id: BodyId) -> Vector3<f32> {
        unsafe {
            let mut result = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetAngularVelocity(self.0, body_id, result.as_mut_ptr());
            Vector3::from(result)
        }
    }

    pub fn get_point_velocity(
        &self,
        body_id: BodyId,
        point: impl Into<Point3<f32>>,
    ) -> Vector3<f32> {
        unsafe {
            let mut result = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetPointVelocity(
                self.0,
                body_id,
                point.into().as_ref().as_ptr(),
                result.as_mut_ptr(),
            );

            Vector3::from(result)
        }
    }

    pub fn position(&self, body_id: BodyId) -> Point3<f32> {
        unsafe {
            let mut result = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetPosition(self.0, body_id, result.as_mut_ptr());
            Point3::from(result)
        }
    }

    pub fn set_position(
        &self,
        body_id: BodyId,
        position: impl Into<Point3<f32>>,
        activation: Activation,
    ) {
        let position = position.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_SetPosition(
                self.0,
                body_id,
                position.as_ptr(),
                activation as _,
            );
        }
    }

    pub fn center_of_mass_position(&self, body_id: BodyId) -> Point3<f32> {
        unsafe {
            let mut result = [0.; 3];
            jolt_sys::JPC_BodyInterface_GetCenterOfMassPosition(
                self.0,
                body_id,
                result.as_mut_ptr(),
            );

            Point3::from(result)
        }
    }

    pub fn rotation(&self, body_id: BodyId) -> Quaternion<f32> {
        unsafe {
            let mut result = [0., 0., 0., 1.];
            jolt_sys::JPC_BodyInterface_GetRotation(self.0, body_id, result.as_mut_ptr());

            Quaternion::from(result)
        }
    }

    pub fn set_rotation(
        &self,
        body_id: BodyId,
        rotation: impl Into<Quaternion<f32>>,
        activation: Activation,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetRotation(
                self.0,
                body_id,
                rotation.into().as_ref().as_ptr(),
                activation as _,
            );
        }
    }

    pub fn set_position_rotation_and_velocity(
        &self,
        body_id: BodyId,
        position: impl Into<Point3<f32>>,
        rotation: impl Into<Quaternion<f32>>,
        linear_velocity: impl Into<Vector3<f32>>,
        angular_velocity: impl Into<Vector3<f32>>,
    ) {
        let position = position.into().to_fixed_vec3();
        let linear_velocity = linear_velocity.into().to_fixed_vec3();
        let angular_velocity = angular_velocity.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_SetPositionRotationAndVelocity(
                self.0,
                body_id,
                position.as_ptr(),
                rotation.into().as_ref().as_ptr(),
                linear_velocity.as_ptr(),
                angular_velocity.as_ptr(),
            );
        }
    }

    pub fn activate_body(&self, body_id: BodyId) {
        unsafe {
            jolt_sys::JPC_BodyInterface_ActivateBody(self.0, body_id);
        }
    }

    pub fn deactivate_body(&self, body_id: BodyId) {
        unsafe {
            jolt_sys::JPC_BodyInterface_DeactivateBody(self.0, body_id);
        }
    }

    pub fn is_active(&self, body_id: BodyId) -> bool {
        unsafe { jolt_sys::JPC_BodyInterface_IsActive(self.0, body_id) }
    }

    pub fn add_force(&self, body_id: BodyId, force: impl Into<Vector3<f32>>) {
        let force = force.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForce(self.0, body_id, force.as_ptr());
        }
    }

    pub fn add_force_at_position(
        &self,
        body_id: BodyId,
        force: impl Into<Vector3<f32>>,
        position: impl Into<Point3<f32>>,
    ) {
        let force = force.into().to_fixed_vec3();
        let position = position.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForceAtPosition(
                self.0,
                body_id,
                force.as_ptr(),
                position.as_ptr(),
            );
        }
    }

    pub fn add_torque(&self, body_id: BodyId, torque: impl Into<Vector3<f32>>) {
        let torque = torque.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddTorque(self.0, body_id, torque.as_ptr());
        }
    }

    pub fn add_force_and_torque(
        &self,
        body_id: BodyId,
        force: impl Into<Vector3<f32>>,
        torque: impl Into<Vector3<f32>>,
    ) {
        let force = force.into().to_fixed_vec3();
        let torque = torque.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForceAndTorque(
                self.0,
                body_id,
                force.as_ptr(),
                torque.as_ptr(),
            );
        }
    }

    pub fn add_impulse(&self, body_id: BodyId, impulse: impl Into<Vector3<f32>>) {
        let impulse = impulse.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddImpulse(self.0, body_id, impulse.as_ptr());
        }
    }

    pub fn add_impulse_at_position(
        &self,
        body_id: BodyId,
        impulse: impl Into<Vector3<f32>>,
        position: impl Into<Point3<f32>>,
    ) {
        let impulse = impulse.into().to_fixed_vec3();
        let position = position.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddImpulseAtPosition(
                self.0,
                body_id,
                impulse.as_ptr(),
                position.as_ptr(),
            );
        }
    }

    pub fn add_angular_impulse(&self, body_id: BodyId, impulse: impl Into<Vector3<f32>>) {
        let impulse = impulse.into().to_fixed_vec3();
        unsafe {
            jolt_sys::JPC_BodyInterface_AddAngularImpulse(self.0, body_id, impulse.as_ptr());
        }
    }

    pub fn set_motion_type(
        &self,
        body_id: BodyId,
        motion_type: MotionType,
        activation: Activation,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetMotionType(
                self.0,
                body_id,
                motion_type as _,
                activation as _,
            );
        }
    }

    // TODO(cohae): This is a bit scary, check the return values to cure my paranoia
    pub fn motion_type(&self, body_id: BodyId) -> MotionType {
        unsafe { transmute(jolt_sys::JPC_BodyInterface_GetMotionType(self.0, body_id)) }
    }

    pub fn set_object_layer(&self, body_id: BodyId, layer: ObjectLayer) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetObjectLayer(self.0, body_id, layer);
        }
    }

    pub fn object_layer(&self, body_id: BodyId) -> ObjectLayer {
        unsafe { jolt_sys::JPC_BodyInterface_GetObjectLayer(self.0, body_id) }
    }
}
