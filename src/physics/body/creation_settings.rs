use crate::{AllowedDOFs, MassProperties, MotionQuality, MotionType, ObjectLayer, Shape, Vec3Ext};
use jolt_sys::{
    JPC_CollisionGroup,  JPC_ObjectLayer,
    JPC_Real, JPC_COLLISION_GROUP_INVALID_GROUP, JPC_COLLISION_GROUP_INVALID_SUB_GROUP,
};
use mint::{Point3, Quaternion, Vector3};
use std::ptr::null;

pub use jolt_sys::{
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA,
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA,
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverrideMassProperties {
    CalculateInertia = JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA as u8,
    CalculateMassAndInertia =
        JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA as u8,
    MassAndInertiaProvided =
        JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED as u8,
}

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
    pub allowed_dofs: AllowedDOFs,
    pub allow_dynamic_or_kinematic: bool,
    pub is_sensor: bool,
    pub collide_kinematic_vs_non_dynamic: bool,
    pub use_manifold_reduction: bool,
    pub apply_gyroscopic_force: bool,
    pub motion_quality: MotionQuality,
    pub enhanced_internal_edge_removal: bool,
    pub allow_sleeping: bool,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub gravity_factor: f32,
    pub num_velocity_steps_override: u32,
    pub num_position_steps_override: u32,
    pub override_mass_properties: OverrideMassProperties,
    pub inertia_multiplier: f32,
    pub mass_properties_override: MassProperties,
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
            allowed_dofs: AllowedDOFs::all(),
            allow_dynamic_or_kinematic: true,
            is_sensor: false,
            collide_kinematic_vs_non_dynamic: false,
            use_manifold_reduction: true,
            apply_gyroscopic_force: false,
            motion_quality: MotionQuality::Discrete,
            enhanced_internal_edge_removal: false,
            allow_sleeping: true,
            friction: 0.2,
            restitution: 0.0,
            linear_damping: 0.05,
            angular_damping: 0.05,
            max_linear_velocity: 500.0,
            max_angular_velocity: 0.25 * std::f32::consts::PI * 60.0,
            gravity_factor: 1.0,
            num_velocity_steps_override: 0,
            num_position_steps_override: 0,
            override_mass_properties: OverrideMassProperties::CalculateMassAndInertia,
            inertia_multiplier: 1.0,
            mass_properties_override: MassProperties::default(),
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
            allowed_dofs: self.allowed_dofs.bits(),
            allow_dynamic_or_kinematic: self.allow_dynamic_or_kinematic,
            is_sensor: self.is_sensor,
            collide_kinematic_vs_non_dynamic: self.collide_kinematic_vs_non_dynamic,
            use_manifold_reduction: self.use_manifold_reduction,
            apply_gyroscopic_force: self.apply_gyroscopic_force,
            motion_quality: self.motion_quality as u8,
            enhanced_internal_edge_removal: self.enhanced_internal_edge_removal,
            allow_sleeping: self.allow_sleeping,
            friction: self.friction,
            restitution: self.restitution,
            linear_damping: self.linear_damping,
            angular_damping: self.angular_damping,
            max_linear_velocity: self.max_linear_velocity,
            max_angular_velocity: self.max_angular_velocity,
            gravity_factor: self.gravity_factor,
            num_velocity_steps_override: self.num_velocity_steps_override,
            num_position_steps_override: self.num_position_steps_override,
            override_mass_properties: self.override_mass_properties as u8,
            inertia_multiplier: self.inertia_multiplier,
            __bindgen_padding_0: 0,
            mass_properties_override: self.mass_properties_override.to_jpc(),
            reserved: std::ptr::null(),
            shape: self.shape.as_raw(),
        }
    }
}
