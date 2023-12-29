use crate::{BroadPhaseLayer, ObjectLayer};

// JPC_ObjectLayerPairFilterVTable
pub trait ObjectLayerPairFilter {
    fn should_collide(&self, object1: ObjectLayer, object2: ObjectLayer) -> bool;
}

// JPC_BroadPhaseLayerInterfaceVTable
pub trait BroadPhaseLayerInterface {
    fn num_broad_phase_layers(&self) -> u32;
    fn broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer;
}

// JPC_ObjectVsBroadPhaseLayerFilterVTable
pub trait ObjectVsBroadPhaseLayerFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool;
}

// JPC_BroadPhaseLayerFilterVTable
pub trait BroadPhaseLayerFilter {
    fn should_collide(&self, layer: BroadPhaseLayer) -> bool;
}
