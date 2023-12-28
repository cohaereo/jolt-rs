use crate::ShapeRef;
use glam::{Quat, Vec3A};
use jolt_sys::{
    JPC_CollisionGroup, JPC_EMotionQuality_JPC_MOTION_QUALITY_DISCRETE,
    JPC_EMotionType_JPC_MOTION_TYPE_DYNAMIC,
    JPC_EOverrideMassProperties_JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA, JPC_MassProperties,
    JPC_MotionQuality, JPC_MotionType, JPC_ObjectLayer, JPC_OverrideMassProperties,
    JPC_COLLISION_GROUP_INVALID_GROUP, JPC_COLLISION_GROUP_INVALID_SUB_GROUP,
};
use std::ptr::null;

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct BodyCreationSettings {
    pub position: Vec3A,
    pub rotation: Quat,
    pub linear_velocity: Vec3A,
    pub angular_velocity: Vec3A,
    pub user_data: u64,
    pub object_layer: JPC_ObjectLayer,
    pub collision_group: JPC_CollisionGroup,
    pub motion_type: JPC_MotionType,
    pub allow_dynamic_or_kinematic: bool,
    pub is_sensor: bool,
    pub use_manifold_reduction: bool,
    pub motion_quality: JPC_MotionQuality,
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
    __bindgen_padding_0: [u64; 0usize],
    pub mass_properties_override: JPC_MassProperties,
    reserved: *const ::std::os::raw::c_void,
    pub shape: ShapeRef,
}

impl BodyCreationSettings {
    pub fn new(
        shape: ShapeRef,
        position: Vec3A,
        rotation: Quat,
        motion_type: JPC_MotionType,
        object_layer: JPC_ObjectLayer,
    ) -> BodyCreationSettings {
        Self {
            shape,
            position,
            rotation,
            motion_type,
            object_layer,
            ..Default::default()
        }
    }

    pub fn as_jpc(&self) -> *const jolt_sys::JPC_BodyCreationSettings {
        self as *const _ as _
    }
}

impl Default for BodyCreationSettings {
    fn default() -> Self {
        Self {
            position: Vec3A::ZERO,
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3A::ZERO,
            angular_velocity: Vec3A::ZERO,
            user_data: 0,
            object_layer: 0,
            collision_group: JPC_CollisionGroup {
                filter: null(),
                group_id: JPC_COLLISION_GROUP_INVALID_GROUP,
                sub_group_id: JPC_COLLISION_GROUP_INVALID_SUB_GROUP,
            },
            motion_type: JPC_EMotionType_JPC_MOTION_TYPE_DYNAMIC as _,
            allow_dynamic_or_kinematic: false,
            is_sensor: false,
            use_manifold_reduction: true,
            motion_quality: JPC_EMotionQuality_JPC_MOTION_QUALITY_DISCRETE as _,
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
            __bindgen_padding_0: [0; 0],
            mass_properties_override: unsafe { std::mem::zeroed() },
            shape: unsafe { std::mem::zeroed() },
            reserved: null(),
        }
    }
}
