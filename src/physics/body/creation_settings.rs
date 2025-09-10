use crate::{MotionQuality, MotionType, ObjectLayer, Shape, Vec3Ext};
use jolt_sys::{
    JPC_AllowedDOFs, JPC_CollisionGroup, JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_ALL, JPC_MassProperties,
    JPC_ObjectLayer, JPC_OverrideMassProperties, JPC_Real, JPC_COLLISION_GROUP_INVALID_GROUP,
    JPC_COLLISION_GROUP_INVALID_SUB_GROUP,
};
use mint::{Point3, Quaternion, Vector3};
use std::ptr::null;

pub use jolt_sys::{
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA,
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA,
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED,
};

#[derive(Clone)]
pub struct BodyCreationSettings {
    pub position: Point3<JPC_Real>,
    pub rotation: Quaternion<f32>,
    pub linear_velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub user_data: u64,
    pub object_layer: ObjectLayer,
    pub collision_group: JPC_CollisionGroup,
    pub motion_type: MotionType,
    pub allowed_dofs: JPC_AllowedDOFs,
    pub allow_dynamic_or_kinematic: bool,
    pub is_sensor: bool,
    pub sensor_detects_static: bool,
    pub use_manifold_reduction: bool,
    pub motion_quality: MotionQuality,
    pub allow_sleeping: bool,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub gravity_factor: f32,
    pub override_mass_properties: JPC_OverrideMassProperties,
    pub inertia_multiplier: f32,
    pub mass_properties_override: JPC_MassProperties,
    pub shape: Shape,
}

impl BodyCreationSettings {
    pub fn new<P, R>(
        shape: Shape,
        position: P,
        rotation: R,
        motion_type: MotionType,
        object_layer: JPC_ObjectLayer,
    ) -> BodyCreationSettings
    where
        P: Into<Point3<JPC_Real>>,
        R: Into<Quaternion<f32>>,
    {
        let p = position.into();
        Self {
            shape,
            position: p.into(),
            rotation: rotation.into(),
            motion_type,
            object_layer,

            linear_velocity: Vector3::from_slice(&[0f32; 4]),
            angular_velocity: Vector3::from_slice(&[0f32; 4]),
            user_data: 0,
            collision_group: JPC_CollisionGroup {
                filter: null(),
                group_id: JPC_COLLISION_GROUP_INVALID_GROUP,
                sub_group_id: JPC_COLLISION_GROUP_INVALID_SUB_GROUP,
            },
            allowed_dofs: JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_ALL as _,
            allow_dynamic_or_kinematic: true,
            is_sensor: false,
            sensor_detects_static: false,
            use_manifold_reduction: true,
            motion_quality: MotionQuality::Discrete,
            allow_sleeping: true,
            friction: 0.2,
            restitution: 0.0,
            linear_damping: 0.05,
            angular_damping: 0.05,
            max_linear_velocity: 500.0,
            max_angular_velocity: 0.25 * std::f32::consts::PI * 60.0,
            gravity_factor: 1.0,
            override_mass_properties:
                JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA as _,
            inertia_multiplier: 1.0,
            mass_properties_override: unsafe { std::mem::zeroed() },
        }
    }

    pub(crate) fn to_jpc(&self) -> jolt_sys::JPC_BodyCreationSettings {
        jolt_sys::JPC_BodyCreationSettings {
            position: self.position.to_fixed_vec3(),
            rotation: self.rotation.into(),
            linear_velocity: self.linear_velocity.to_fixed_vec3(),
            angular_velocity: self.angular_velocity.to_fixed_vec3(),
            user_data: self.user_data,
            object_layer: self.object_layer,
            collision_group: self.collision_group,
            motion_type: self.motion_type as u8,
            allowed_dofs: self.allowed_dofs,
            allow_dynamic_or_kinematic: self.allow_dynamic_or_kinematic,
            is_sensor: self.is_sensor,
            sensor_detects_static: self.sensor_detects_static,
            use_manifold_reduction: self.use_manifold_reduction,
            motion_quality: self.motion_quality as u8,
            allow_sleeping: self.allow_sleeping,
            friction: self.friction,
            restitution: self.restitution,
            linear_damping: self.linear_damping,
            angular_damping: self.angular_damping,
            max_linear_velocity: self.max_linear_velocity,
            max_angular_velocity: self.max_angular_velocity,
            gravity_factor: self.gravity_factor,
            override_mass_properties: self.override_mass_properties,
            inertia_multiplier: self.inertia_multiplier,
            __bindgen_padding_0: [0; 0],
            mass_properties_override: self.mass_properties_override,
            reserved: std::ptr::null(),
            shape: self.shape.as_raw(),
        }
    }
}
