use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AllowedDOFs: u8 {
        const TRANSLATION_X = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_TRANSLATIONX as u8;
        const TRANSLATION_Y = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_TRANSLATIONY as u8;
        const TRANSLATION_Z = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_TRANSLATIONZ as u8;
        const ROTATION_X = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_ROTATIONX as u8;
        const ROTATION_Y = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_ROTATIONY as u8;
        const ROTATION_Z = jolt_sys::JPC_EAllowedDOFs_JPC_ALLOWED_DOFS_ROTATIONZ as u8;
        const ALL_TRANSLATIONS = Self::TRANSLATION_X.bits() | Self::TRANSLATION_Y.bits() | Self::TRANSLATION_Z.bits();
        const ALL_ROTATIONS = Self::ROTATION_X.bits() | Self::ROTATION_Y.bits() | Self::ROTATION_Z.bits();
    }
}
