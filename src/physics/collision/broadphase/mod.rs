mod vtable;

use jolt_sys::{JPC_BroadPhaseLayer, JPC_ObjectLayer};
use std::ffi::c_void;
pub use vtable::*;

pub type BroadPhaseLayer = JPC_BroadPhaseLayer;
pub type ObjectLayer = JPC_ObjectLayer;

#[repr(C)]
pub struct BroadPhaseLayerInterfaceWrapper {
    pub(crate) vtable: *const jolt_sys::JPC_BroadPhaseLayerInterfaceVTable,
    pub(crate) inner: Box<dyn BroadPhaseLayerInterface>,
}

impl BroadPhaseLayerInterfaceWrapper {
    pub fn new(inner: Box<dyn BroadPhaseLayerInterface>) -> Self {
        let vtable = Box::new(jolt_sys::JPC_BroadPhaseLayerInterfaceVTable {
            __vtable_header: unsafe { std::mem::zeroed() },
            GetNumBroadPhaseLayers: Some(Self::get_num_broad_phase_layers),
            GetBroadPhaseLayer: Some(Self::get_broad_phase_layer),
        });

        Self {
            vtable: Box::into_raw(vtable),
            inner,
        }
    }

    unsafe extern "C" fn get_num_broad_phase_layers(wrapper: *const c_void) -> u32 {
        (*(wrapper as *const Self)).inner.num_broad_phase_layers()
    }

    unsafe extern "C" fn get_broad_phase_layer(
        wrapper: *const c_void,
        layer_out: *mut JPC_BroadPhaseLayer,
        layer: JPC_ObjectLayer,
    ) -> *const JPC_BroadPhaseLayer {
        *layer_out = (*(wrapper as *const Self)).inner.broad_phase_layer(layer);
        layer_out
    }
}

#[repr(C)]
pub struct ObjectVsBroadPhaseLayerFilterWrapper {
    pub(crate) vtable: *const jolt_sys::JPC_ObjectVsBroadPhaseLayerFilterVTable,
    pub(crate) inner: Box<dyn ObjectVsBroadPhaseLayerFilter>,
}

impl ObjectVsBroadPhaseLayerFilterWrapper {
    pub fn new(inner: Box<dyn ObjectVsBroadPhaseLayerFilter>) -> Self {
        let vtable = Box::new(jolt_sys::JPC_ObjectVsBroadPhaseLayerFilterVTable {
            __vtable_header: unsafe { std::mem::zeroed() },
            ShouldCollide: Some(Self::should_collide),
        });

        Self {
            vtable: Box::into_raw(vtable),
            inner,
        }
    }

    unsafe extern "C" fn should_collide(
        wrapper: *const c_void,
        layer1: JPC_ObjectLayer,
        layer2: JPC_BroadPhaseLayer,
    ) -> bool {
        (*(wrapper as *const Self))
            .inner
            .should_collide(layer1, layer2)
    }
}

#[repr(C)]
pub struct ObjectLayerPairFilterWrapper {
    pub(crate) vtable: *const jolt_sys::JPC_ObjectLayerPairFilterVTable,
    pub(crate) inner: Box<dyn ObjectLayerPairFilter>,
}

impl ObjectLayerPairFilterWrapper {
    pub fn new(inner: Box<dyn ObjectLayerPairFilter>) -> Self {
        let vtable = Box::new(jolt_sys::JPC_ObjectLayerPairFilterVTable {
            __vtable_header: unsafe { std::mem::zeroed() },
            ShouldCollide: Some(Self::should_collide),
        });

        Self {
            vtable: Box::into_raw(vtable),
            inner,
        }
    }

    unsafe extern "C" fn should_collide(
        wrapper: *const c_void,
        object1: JPC_ObjectLayer,
        object2: JPC_ObjectLayer,
    ) -> bool {
        (*(wrapper as *const Self))
            .inner
            .should_collide(object1, object2)
    }
}
