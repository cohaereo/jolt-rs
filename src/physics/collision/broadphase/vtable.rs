use jolt_sys::{JPC_BroadPhaseLayer, JPC_ObjectLayer};

// JPC_ObjectLayerPairFilterVTable
pub trait ObjectLayerPairFilter {
    fn should_collide(&self, object1: JPC_ObjectLayer, object2: JPC_ObjectLayer) -> bool;
}

// JPC_BroadPhaseLayerInterfaceVTable
pub trait BroadPhaseLayerInterface {
    fn num_broad_phase_layers(&self) -> u32;
    fn broad_phase_layer(&self, layer: JPC_ObjectLayer) -> JPC_BroadPhaseLayer;
}

// JPC_ObjectVsBroadPhaseLayerFilterVTable
pub trait ObjectVsBroadPhaseLayerFilter {
    fn should_collide(&self, layer1: JPC_ObjectLayer, layer2: JPC_BroadPhaseLayer) -> bool;
}

// JPC_BroadPhaseLayerFilterVTable
pub trait BroadPhaseLayerFilter {
    fn should_collide(&self, layer: JPC_BroadPhaseLayer) -> bool;
}
