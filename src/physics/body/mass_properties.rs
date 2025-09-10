use mint::ColumnMatrix4;

#[derive(Debug, Clone, Copy)]
pub struct MassProperties {
    pub mass: f32,
    pub inertia: ColumnMatrix4<f32>,
}

impl MassProperties {
    pub(crate) fn to_jpc(&self) -> jolt_sys::JPC_MassProperties {
        jolt_sys::JPC_MassProperties {
            mass: self.mass,
            __bindgen_padding_0: [0; 3],
            inertia: self.inertia.into(),
        }
    }
}

impl Default for MassProperties {
    fn default() -> Self {
        Self {
            mass: 0.0,
            inertia: ColumnMatrix4 {
                x: [0.0, 0.0, 0.0, 0.0].into(),
                y: [0.0, 0.0, 0.0, 0.0].into(),
                z: [0.0, 0.0, 0.0, 0.0].into(),
                w: [0.0, 0.0, 0.0, 0.0].into(),
            },
        }
    }
}
