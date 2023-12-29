use jolt::ShapeSettings;
use rand::random;
use std::num::NonZeroUsize;
use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Jolt Samples".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    jolt::register_default_allocator();
    jolt::create_factory();
    jolt::register_types();

    let mut temp_allocator = jolt::TempAllocator::create(10 * 1024 * 1024);
    let mut job_system = jolt::JobSystem::create(
        jolt::JobSystem::MAX_PHYSICS_JOBS,
        jolt::JobSystem::MAX_PHYSICS_BARRIERS,
        (std::thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(4).unwrap())
            .get()
            - 1)
        .clamp(1, 16) as _,
    );

    let broad_phase_layer_interface = BroadPhaseLayerInterfaceImpl;
    let object_vs_broadphase_layer_filter = ObjectVsBroadPhaseLayerFilterImpl;
    let object_vs_object_layer_filter = OLayerPairInterfaceImpl;

    let physics_system = jolt::PhysicsSystem::create(
        1024,
        0,
        65536,
        10240,
        Box::new(broad_phase_layer_interface),
        Box::new(object_vs_broadphase_layer_filter),
        Box::new(object_vs_object_layer_filter),
    );

    let body_interface = physics_system.get_body_interface();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 15.0, 15.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    // sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));
    let mut spheres = vec![];
    for i in 0..10 {
        let mut sphere = Gm::new(
            Mesh::new(&context, &CpuMesh::sphere(16)),
            PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                    ..Default::default()
                },
            ),
        );

        let deviation_x = (random::<f32>() * 0.5 + 0.5) * 0.01;
        let deviation_y = (random::<f32>() * 0.5 + 0.5) * 0.01;

        let sphere_settings = jolt::BodyCreationSettings::new(
            jolt::SphereShape::create(1.0),
            glam::vec3a(deviation_x, 2.0 + (i as f32 * 3.0), deviation_y),
            glam::Quat::IDENTITY,
            jolt::MotionType::Dynamic,
            OLAYER_MOVING,
        );
        let sphere_id =
            body_interface.create_and_add_body(&sphere_settings, jolt::Activation::Activate);

        spheres.push((sphere, sphere_id));
    }

    let floor_shape_settings = jolt::BoxShapeSettings::create(glam::Vec3::new(100.0, 1.0, 100.0));

    // Create the shape
    let floor_shape = floor_shape_settings
        .create_shape()
        .expect("Failed to create floor shape");

    let floor_settings = jolt::BodyCreationSettings::new(
        floor_shape,
        glam::vec3a(0.0, -1.0, 0.0),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    // Create the actual rigid body
    let floor = body_interface
        .create_body(&floor_settings)
        .expect("Failed to create body"); // Note that if we run out of bodies this can return None

    // Add it to the world
    body_interface.add_body(unsafe { (*floor).id }, jolt::Activation::DontActivate);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    window.render_loop(move |mut frame_input| {
        let delta_time = frame_input.elapsed_time / 1000.;
        physics_system.update(
            delta_time as f32,
            1,
            1,
            &mut temp_allocator,
            &mut job_system,
        );

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        for (sphere, sphere_id) in spheres.iter_mut() {
            let position = body_interface.center_of_mass_position(*sphere_id);
            let rotation = body_interface.rotation(*sphere_id);
            sphere.set_transformation(
                Mat4::from_translation(Vec3::from(position.to_array()))
                    * Mat4::from(Quat::from(rotation.to_array()))
                    * Mat4::from_scale(1.0),
            );
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                spheres.iter().map(|(s, _)| s).into_iter(),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}

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

pub struct BroadPhaseLayerInterfaceImpl;
impl jolt::BroadPhaseLayerInterface for BroadPhaseLayerInterfaceImpl {
    fn num_broad_phase_layers(&self) -> u32 {
        BLAYER_COUNT
    }

    fn broad_phase_layer(&self, layer: jolt::ObjectLayer) -> jolt::BroadPhaseLayer {
        const OBJECT_TO_BROADPHASE: [jolt::BroadPhaseLayer; 2] = [BLAYER_NON_MOVING, BLAYER_MOVING];

        OBJECT_TO_BROADPHASE[layer as usize]
    }
}
