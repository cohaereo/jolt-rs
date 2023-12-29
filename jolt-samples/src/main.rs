use rand::random;
use std::num::NonZeroUsize;
use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Jolt Samples".to_string(),
        max_size: Some((1280, 720)),
        surface_settings: SurfaceSettings {
            vsync: false,
            ..Default::default()
        },
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
        10240,
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
        vec3(25.0, 25.0, 25.0),
        vec3(0.0, 8.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    const WALL_OFFSET: f32 = 8.0;
    const WALL_THICKNESS: f32 = 4.0;

    // Bottom plane
    let floor_settings = jolt::BodyCreationSettings::new(
        jolt::BoxShape::create(glam::Vec3::new(100.0, WALL_THICKNESS, 100.0)),
        glam::vec3a(0.0, -1.0, 0.0),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    body_interface.create_and_add_body(&floor_settings, jolt::Activation::DontActivate);

    // Left plane
    let floor_settings = jolt::BodyCreationSettings::new(
        jolt::BoxShape::create(glam::Vec3::new(WALL_THICKNESS, 100.0, 100.0)),
        glam::vec3a(-WALL_OFFSET, 0.0, 0.0),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    body_interface.create_and_add_body(&floor_settings, jolt::Activation::DontActivate);

    // Right plane
    let floor_settings = jolt::BodyCreationSettings::new(
        jolt::BoxShape::create(glam::Vec3::new(WALL_THICKNESS, 100.0, 100.0)),
        glam::vec3a(WALL_OFFSET, 0.0, 0.0),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    body_interface.create_and_add_body(&floor_settings, jolt::Activation::DontActivate);

    // Front plane
    let floor_settings = jolt::BodyCreationSettings::new(
        jolt::BoxShape::create(glam::Vec3::new(100.0, 100.0, WALL_THICKNESS)),
        glam::vec3a(0.0, 0.0, -WALL_OFFSET),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    body_interface.create_and_add_body(&floor_settings, jolt::Activation::DontActivate);

    // Back plane
    let floor_settings = jolt::BodyCreationSettings::new(
        jolt::BoxShape::create(glam::Vec3::new(100.0, 100.0, WALL_THICKNESS)),
        glam::vec3a(0.0, 0.0, WALL_OFFSET),
        glam::Quat::IDENTITY,
        jolt::MotionType::Static,
        OLAYER_NON_MOVING,
    );

    body_interface.create_and_add_body(&floor_settings, jolt::Activation::DontActivate);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::sphere(16)),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    let mut spheres = vec![];
    let mut sphere_quota = 2600;

    let mut collision_steps = 3;

    let mut gui = GUI::new(&context);
    window.render_loop(move |mut frame_input| {
        if sphere_quota > 0 {
            for _ in 0..8 {
                let deviation_x = random::<f32>() * 0.5 + 0.5;
                let deviation_y = random::<f32>() * 0.5 + 0.5;

                let sphere_settings = jolt::BodyCreationSettings::new(
                    jolt::SphereShape::create(0.25),
                    glam::vec3a(deviation_x, 20.0, deviation_y),
                    glam::Quat::IDENTITY,
                    jolt::MotionType::Dynamic,
                    OLAYER_MOVING,
                );
                let sphere_id = body_interface
                    .create_and_add_body(&sphere_settings, jolt::Activation::Activate);
                body_interface.set_linear_velocity(sphere_id, glam::vec3(0.0, -10.0, 0.0));

                spheres.push((sphere_id, (random::<[u8; 3]>())));

                sphere_quota -= 1;
            }

            println!("Sphere quota: {}", sphere_quota);
        }

        let delta_time = frame_input.elapsed_time / 1000.;
        physics_system.update(
            delta_time as f32,
            collision_steps,
            1,
            &mut temp_allocator,
            &mut job_system,
        );

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Debug Panel");
                    ui.label(format!("FPS: {:.1}", 1000.0 / frame_input.elapsed_time));
                    ui.label(format!("Spheres: {}", spheres.len()));
                    ui.label(format!("Remaining spheres: {}", sphere_quota));
                    ui.separator();
                    ui.add(Slider::new(&mut collision_steps, 1..=10).text("Collision steps"));
                    ui.label("Collision steps determines the number of iterations dedicated to detecting and resolving collisions for a slider in the physics application.\nHigher values will result in more accurate collisions, but are also more computationally expensive.");
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        instanced_mesh.set_instances(&Instances {
            transformations: spheres
                .iter()
                .map(|(sphere_id, _)| {
                    let position = body_interface.center_of_mass_position(*sphere_id);
                    let rotation = body_interface.rotation(*sphere_id);
                    Mat4::from_translation(Vec3::from(position.to_array()))
                        * Mat4::from(Quat::from(rotation.to_array()))
                        * Mat4::from_scale(0.25)
                })
                .collect::<Vec<_>>(),
            colors: Some(
                spheres
                    .iter()
                    .map(|(_, color)| Srgba::from(*color))
                    .collect::<Vec<_>>(),
            ),
            ..Default::default()
        });

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, instanced_mesh.into_iter(), &[&light0, &light1]);

        frame_input.screen().write(|| gui.render());

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
