use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct ConfigurationFeatures: u32 {
        const DOUBLE_PRECISION              = jolt_sys::JPC_EFeatures_JPC_FEATURE_DOUBLE_PRECISION as u32;
        const NEON                          = jolt_sys::JPC_EFeatures_JPC_FEATURE_NEON as u32;
        const SSE                           = jolt_sys::JPC_EFeatures_JPC_FEATURE_SSE as u32;
        const SSE4_1                        = jolt_sys::JPC_EFeatures_JPC_FEATURE_SSE4_1 as u32;
        const SSE4_2                        = jolt_sys::JPC_EFeatures_JPC_FEATURE_SSE4_2 as u32;
        const AVX                           = jolt_sys::JPC_EFeatures_JPC_FEATURE_AVX as u32;
        const AVX2                          = jolt_sys::JPC_EFeatures_JPC_FEATURE_AVX2 as u32;
        const AVX512                        = jolt_sys::JPC_EFeatures_JPC_FEATURE_AVX512 as u32;
        const F16C                          = jolt_sys::JPC_EFeatures_JPC_FEATURE_F16C as u32;
        const LZCNT                         = jolt_sys::JPC_EFeatures_JPC_FEATURE_LZCNT as u32;
        const TZCNT                         = jolt_sys::JPC_EFeatures_JPC_FEATURE_TZCNT as u32;
        const FMADD                         = jolt_sys::JPC_EFeatures_JPC_FEATURE_FMADD as u32;
        const PLATFORM_DETERMINISTIC        = jolt_sys::JPC_EFeatures_JPC_FEATURE_PLATFORM_DETERMINISTIC as u32;
        const FLOATING_POINT_EXCEPTIONS     = jolt_sys::JPC_EFeatures_JPC_FEATURE_FLOATING_POINT_EXCEPTIONS as u32;
        const DEBUG                         = jolt_sys::JPC_EFeatures_JPC_FEATURE_DEBUG as u32;
    }
}

pub fn get_features() -> ConfigurationFeatures {
    ConfigurationFeatures::from_bits_retain(unsafe { jolt_sys::JPC_GetFeatures() })
}
