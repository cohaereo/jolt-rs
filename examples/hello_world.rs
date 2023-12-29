use glam::{vec3, vec3a, Quat, Vec3};
use jolt::ShapeSettings;

const OLAYER_NON_MOVING: jolt::ObjectLayer = 0;
const OLAYER_MOVING: jolt::ObjectLayer = 1;

pub struct OLayerPairInterfaceImpl;
impl jolt::ObjectLayerPairFilter for OLayerPairInterfaceImpl {
    fn should_collide(&self, object1: jolt::ObjectLayer, object2: jolt::ObjectLayer) -> bool {
        match object1 {
            OLAYER_NON_MOVING => object2 == OLAYER_MOVING,
            OLAYER_MOVING => true,
            u => panic!("Unknown ObjectLayer({u}) passed"),
        }
    }
}

const BLAYER_NON_MOVING: jolt::BroadPhaseLayer = 0;
const BLAYER_MOVING: jolt::BroadPhaseLayer = 1;
const BLAYER_COUNT: u32 = 2;

pub struct ObjectVsBroadPhaseLayerFilterImpl;
impl jolt::ObjectVsBroadPhaseLayerFilter for ObjectVsBroadPhaseLayerFilterImpl {
    fn should_collide(&self, layer1: jolt::ObjectLayer, layer2: jolt::BroadPhaseLayer) -> bool {
        match layer1 {
            OLAYER_NON_MOVING => layer2 == BLAYER_MOVING,
            OLAYER_MOVING => true,
            u => panic!("Unknown ObjectLayer({u}) passed"),
        }
    }
}

pub struct BpLayerInterfaceImpl;
impl jolt::BroadPhaseLayerInterface for BpLayerInterfaceImpl {
    fn num_broad_phase_layers(&self) -> u32 {
        BLAYER_COUNT
    }

    fn broad_phase_layer(&self, layer: jolt::ObjectLayer) -> jolt::BroadPhaseLayer {
        const OBJECT_TO_BROADPHASE: [jolt::BroadPhaseLayer; 2] = [BLAYER_NON_MOVING, BLAYER_MOVING];

        OBJECT_TO_BROADPHASE[layer as usize]
    }
}

pub struct MyContactListener;
impl jolt::ContactListener for MyContactListener {
    fn on_contact_validate(
        &self,
        _body1: &jolt::Body,
        _body2: &jolt::Body,
        _base_offset: Vec3,
        _collision_result: &jolt::CollideShapeResult,
    ) -> jolt::ValidateResult {
        println!("Contact validate callback");
        jolt::ValidateResult::AcceptAllContactsForThisBodyPair
    }

    fn on_contact_added(
        &self,
        _body1: &jolt::Body,
        _body2: &jolt::Body,
        _manifold: &jolt::ContactManifold,
        _io_settings: &mut jolt::ContactSettings,
    ) {
        println!("A contact was added");
    }

    fn on_contact_persisted(
        &self,
        _body1: &jolt::Body,
        _body2: &jolt::Body,
        _manifold: &jolt::ContactManifold,
        _io_settings: &mut jolt::ContactSettings,
    ) {
        println!("A contact was persisted");
    }

    fn on_contact_removed(&self, _sub_shape_pair: &jolt::SubShapeIDPair) {
        println!("A contact was removed");
    }
}

pub struct MyBodyActivationListener;
impl jolt::BodyActivationListener for MyBodyActivationListener {
    fn on_body_activated(&self, _body: jolt::BodyId, _user_data: u64) {
        println!("A body was activated");
    }

    fn on_body_deactivated(&self, _body: jolt::BodyId, _user_data: u64) {
        println!("A body went to sleep");
    }
}

// This is the max amount of rigid bodies that you can add to the physics system. If you try to add more you'll get an error.
// Note: This value is low because this is a simple test. For a real project use something in the order of 65536.
const MAX_BODIES: u32 = 1024;

// This determines how many mutexes to allocate to protect rigid bodies from concurrent access. Set it to 0 for the default settings.
const NUM_BODY_MUTEXES: u32 = 0;

// This is the max amount of body pairs that can be queued at any time (the broad phase will detect overlapping
// body pairs based on their bounding boxes and will insert them into a queue for the narrowphase). If you make this buffer
// too small the queue will fill up and the broad phase jobs will start to do narrow phase work. This is slightly less efficient.
// Note: This value is low because this is a simple test. For a real project use something in the order of 65536.
const MAX_BODY_PAIRS: u32 = 1024;

// This is the maximum size of the contact constraint buffer. If more contacts (collisions between bodies) are detected than this
// number then these contacts will be ignored and bodies will start interpenetrating / fall through the world.
// Note: This value is low because this is a simple test. For a real project use something in the order of 10240.
const MAX_CONTACT_CONSTRAINTS: u32 = 1024;

fn main() {
    println!("Features: {:?}", jolt::get_features());

    jolt::register_default_allocator();
    jolt::create_factory();
    jolt::register_types();

    let mut temp_allocator = jolt::TempAllocator::create(10 * 1024 * 1024);
    let mut job_system = jolt::JobSystem::create(
        jolt::JobSystem::MAX_PHYSICS_JOBS,
        jolt::JobSystem::MAX_PHYSICS_BARRIERS,
        (num_cpus::get() - 1).clamp(1, 16) as _,
    );

    let broad_phase_layer_interface = BpLayerInterfaceImpl;
    let object_vs_broadphase_layer_filter = ObjectVsBroadPhaseLayerFilterImpl;
    let object_vs_object_layer_filter = OLayerPairInterfaceImpl;

    let physics_system = jolt::PhysicsSystem::create(
        MAX_BODIES,
        NUM_BODY_MUTEXES,
        MAX_BODY_PAIRS,
        MAX_CONTACT_CONSTRAINTS,
        Box::new(broad_phase_layer_interface),
        Box::new(object_vs_broadphase_layer_filter),
        Box::new(object_vs_object_layer_filter),
    );

    let body_interface = physics_system.get_body_interface();

    // A body activation listener gets not   ified when bodies activate and go to sleep
    // Note that this is called from a job so whatever you do here needs to be thread safe.
    // Registering one is entirely optional.
    let body_activation_listener = MyBodyActivationListener;
    physics_system.set_body_activation_listener(Box::new(body_activation_listener));

    // A contact listener gets notified when bodies (are about to) collide, and when they separate again.
    // Note that this is called from a job so whatever you do here needs to be thread safe.
    // Registering one is entirely optional.
    let contact_listener = MyContactListener;
    physics_system.set_contact_listener(Box::new(contact_listener));

    // Next we can create a rigid body to serve as the floor, we make a large box
    // Create the settings for the collision volume (the shape).
    // Note that for simple shapes (like boxes) you can also directly construct a BoxShape.
    let floor_shape_settings = jolt::BoxShapeSettings::create(Vec3::new(100.0, 1.0, 100.0));

    // Create the shape
    let floor_shape = floor_shape_settings
        .create_shape()
        .expect("Failed to create floor shape");

    // Create the settings for the body itself. Note that here you can also set other properties like the restitution / friction.
    let floor_settings = jolt::BodyCreationSettings::new(
        floor_shape,
        vec3a(0.0, -1.0, 0.0),
        Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    // Create the actual rigid body
    let floor = body_interface
        .create_body(&floor_settings)
        .expect("Failed to create body"); // Note that if we run out of bodies this can return None

    // Add it to the world
    body_interface.add_body(unsafe { (*floor).id }, jolt::Activation::DontActivate);

    // Now create a dynamic body to bounce on the floor
    // Note that this uses the shorthand version of creating and adding a body to the world
    let sphere_settings = jolt::BodyCreationSettings::new(
        jolt::SphereShape::create(0.5),
        vec3a(0.0, 2.0, 0.0),
        Quat::IDENTITY,
        jolt::MotionType::Dynamic,
        OLAYER_MOVING,
    );
    let sphere_id =
        body_interface.create_and_add_body(&sphere_settings, jolt::Activation::Activate);

    // Now you can interact with the dynamic body, in this case we're going to give it a velocity.
    // (note that if we had used CreateBody then we could have set the velocity straight on the body before adding it to the physics system)
    body_interface.set_linear_velocity(sphere_id, vec3(0.0, -5.0, 0.0));

    let mut step = 0;
    while body_interface.is_active(sphere_id) {
        step += 1;

        let position = body_interface.center_of_mass_position(sphere_id);
        let velocity = body_interface.linear_velocity(sphere_id);
        println!("Step {step}: Position = {position:?}, Velocity = {velocity:?}");

        // If you take larger steps than 1 / 60th of a second you need to do multiple collision steps in order to keep the simulation stable. Do 1 collision step per 1 / 60th of a second (round up).
        const COLLISION_STEPS: i32 = 1;

        // If you want more accurate step results you can do multiple sub steps within a collision step. Usually you would set this to 1.
        const INTEGRATION_SUBSTEPS: i32 = 1;

        physics_system.update(
            1. / 60.,
            COLLISION_STEPS,
            INTEGRATION_SUBSTEPS,
            &mut temp_allocator,
            &mut job_system,
        );
    }
}
