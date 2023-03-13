use bevy::prelude::*;
use bevy_stl::StlPlugin;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin, MovementSettings};
use bevy_inspector_egui::WorldInspectorPlugin;

use particle_physic_sim::{ParticlePlugin, EnviromentPlugin, CAM_X, CAM_Y, CAM_Z, HEIGHT, WIDTH};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Particle Simulator".to_string(),
            resizable: false,
            ..Default::default()
        },
        ..default()
        }))
    .add_plugin(StlPlugin)
    .add_startup_system(setup_light)
    .add_startup_system(setup_camera)
    .add_startup_system(setup_floor)
    .add_plugin(ParticlePlugin)
    .add_plugin(EnviromentPlugin)
    .add_plugin(NoCameraPlayerPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .run();
}

fn setup_light(mut commands: Commands) {
    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

fn setup_camera(
    mut commands: Commands,
    mut settings: ResMut<MovementSettings>,
    ) {
    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(CAM_X, CAM_Y, CAM_Z).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn(camera).insert(FlyCam);

    settings.speed *= 20.0;
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });
}
