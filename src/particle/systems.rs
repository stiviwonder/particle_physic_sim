use super::entities::*;
use bevy::prelude::*;
use bevy_flycam::FlyCam;

use super::consts::*;

// TODO: ahora de esto se encarga el modulo enviroment
pub fn startup_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut offsize = 0.;
    let mut par_pos = ParticlePositions { vec: Vec::new() };
    let mut par_vels = ParticleVelocities { vec: Vec::new() };

    let mut init_pos = Vec3 {
        x: -10.,
        y: 5.0,
        z: -10.,
    };

    for i in 0..NUM_PAR {
        let p = Particle::new(
            i,
            0,
            false,
            Vec3::new(
                init_pos.x + offsize,
                init_pos.y,
                init_pos.z,
            ),
            Vec3::ZERO,
            Attraction::default(),
            Repulsion::default(),
        );

        println!("Particle pos: {}", p.pos);
        par_pos.vec.push(p.pos);
        par_vels.vec.push(p.vel);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: p.radius,
                    subdivisions: SUBDIV,
                })),
                material: materials.add(Color::CYAN.into()),
                transform: Transform::from_xyz(p.pos.x, p.pos.y, p.pos.z),
                ..default()
            })
            .insert(p);

        if i % 10 == 0 {
            init_pos.z += P_RAD+1.0;
            offsize = 0.;
        } else {
            offsize += P_RAD+1.;
        }
    }

    commands.insert_resource(par_pos);
    commands.insert_resource(par_vels);
}

pub fn get_distance(p1: Vec3, p2: Vec3) -> f32 {
    //          p - center_p
    let x = (p1.x - p2.x) * (p1.x - p2.x);
    let y = (p1.y - p2.y) * (p1.y - p2.y);
    let z = (p1.z - p2.z) * (p1.z - p2.z);

    return (x + y + z).sqrt();
}

pub fn add_gravity(vel: &mut Vec3) {
    *vel -= Vec3::Y * GRAVITY;
}

// NOTE: Esto probablemente tambien haya que cambiar
pub fn get_new_pos(
    time: Res<Time>,
    particles: Query<&Particle>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vels: ResMut<ParticleVelocities>,
) {
    let mut i = 0;

    for p1 in particles.iter() {
        let mut new_vel = p1.vel;

        // process particle interactions
        for p2 in particles.iter() {
            if p1.id != p2.id && !p1.locked {
                let dis = get_distance(p1.pos, p2.pos);

                if dis <= p2.rep.radius {
                    new_vel += p2.repulse(p1);
                } else if dis <= p2.atr.radius && p1.grp == p2.grp {
                    new_vel += p2.attract(p1);
                }
            }
        }

        if !p1.locked {
            add_gravity(&mut new_vel);

            if p1.on_floor() {
                new_vel.y = new_vel.y.abs() * FLOOR_F;
            }

            new_vel *= AIR_F;

            let new_pos = p1.pos + new_vel * time.delta_seconds();

            // Update resources for sync
            par_pos.vec[i] = new_pos;
            par_vels.vec[i] = new_vel;
        }
        i += 1;
    }
}

pub fn sync_particle_data(
    mut particles: Query<&mut Particle>,
    par_pos: Res<ParticlePositions>,
    par_vels: Res<ParticleVelocities>,
) {
    let mut i = 0;
    for mut p in particles.iter_mut() {
        p.pos = par_pos.vec[i];
        p.vel = par_vels.vec[i];
        i += 1;
    }
}

pub fn render_particle_sim(mut query: Query<(&mut Transform, With<Particle>, &Particle)>) {
    for (mut t, _, p) in query.iter_mut() {
        t.translation = p.pos;
    }
}

pub fn shoot_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kb: Res<Input<KeyCode>>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vels: ResMut<ParticleVelocities>,
    cam_pos: Query<&Transform, With<FlyCam>>,
) {
    if kb.pressed(KeyCode::F) {
        let i = par_pos.vec.len();

        for cam in cam_pos.get_single() {
            let local_z = cam.local_z();

            let init_vel = local_z - local_z * 100.;

            let p = Particle::new(i, 1, false, cam.translation, init_vel, Attraction::default(), Repulsion::default());
            par_pos.vec.push(p.pos);
            par_vels.vec.push(p.vel);

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: p.radius,
                        subdivisions: SUBDIV,
                    })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_xyz(p.pos.x, p.pos.y, p.pos.z),
                    ..default()
                })
                .insert(p);
        }
    }
}

pub fn spawn_locked_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vels: ResMut<ParticleVelocities>,
    kb: Res<Input<KeyCode>>,
    cam_pos: Query<&Transform, With<FlyCam>>,
) {

    if kb.just_pressed(KeyCode::B) {
        let i = par_pos.vec.len();

        for cam in cam_pos.get_single() {

            let p = Particle::new(i, 2, true, cam.translation, Vec3::ZERO, Attraction::new(0., 0.), Repulsion::new(P_RAD+1., 200.));
            par_pos.vec.push(p.pos);
            par_vels.vec.push(p.vel);

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: p.radius,
                        subdivisions: SUBDIV,
                    })),
                    material: materials.add(Color::GRAY.into()),
                    transform: Transform::from_xyz(p.pos.x, p.pos.y, p.pos.z),
                    ..default()
                })
                .insert(p);
        }
    }
}

