use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::f32::consts::PI;
//use bevy_stl::StlPlugin;
//use bevy_flycam::{FlyCam, NoCameraPlayerPlugin, MovementSettings};

use particle_physic_sim::{ParticlePlugin, EnviromentPlugin, CAM_X, CAM_Y, CAM_Z};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
//        .add_plugin(StlPlugin)
        .add_startup_system(setup_light)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_floor)
        .add_plugin(ParticlePlugin)
        .add_plugin(EnviromentPlugin)
//        .add_plugin(NoCameraPlayerPlugin)
        .run();
}

fn setup_light(mut commands: Commands) {

        // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        }
        .into(),
        ..default()
    });
}

fn setup_camera(
    mut commands: Commands,
//    mut settings: ResMut<MovementSettings>,
    ) {
    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(CAM_X, CAM_Y, CAM_Z).looking_at(Vec3::new(100., 0., 100.), Vec3::Y),
        ..Default::default()
    };

    // add plugin
    commands.spawn(camera);
//        .insert(FlyCam);

//    settings.speed *= 20.0;
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(50.0))),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });
}
