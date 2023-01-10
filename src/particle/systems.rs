use crate::enviroment::entities::Chunk;

use super::entities::*;
use bevy::prelude::*;
use bevy_flycam::FlyCam;

use super::consts::*;


fn add_gravity(vel: &mut Vec3) {
    *vel -= Vec3::Y * GRAVITY;
}

// NOTE: Esto probablemente tambien haya que cambiar
pub fn get_new_pos(
    time: Res<Time>,
    particles: Query<&Particle>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vel: ResMut<ParticleVelocities>,
) {
    let mut i = 0;

    for p1 in particles.iter() {
        let mut new_vel = p1.vel;

        // process particle interactions
        for p2 in particles.iter() {
            if p1.id != p2.id && !p1.locked {
                let dis = p1.get_distance(p2);

                if dis <= p2.rep.radius {
                    new_vel += p2.repulse(p1);
                } else if dis <= p2.atr.radius && p1.grp == p2.grp {
                    new_vel += p2.attract(p1);
                }
            }
        }

        if !p1.locked {
            add_gravity(&mut new_vel);

            if p1.on_floor(0.) {
                new_vel.y = new_vel.y.abs() * FLOOR_F;
            }

            new_vel *= AIR_F;

            let new_pos = p1.pos + new_vel * time.delta_seconds();

            // Update resources for sync
            par_pos.vec[i] = new_pos;
            par_vel.vec[i] = new_vel;
        }
        i += 1;
    }
}

pub fn sync_particle_data(
    chunk: Res<Chunk>,
    mut par_pos:  ResMut<ParticlePositions>,
    mut par_vel: ResMut<ParticleVelocities>,
) {
    let mut i = 0;
    for cell in chunk.cells.iter() {
        for p in cell.parvec.iter() {
//            println!("PID {}, pos {}\t par_pos = {}", p.id, p.pos, par_pos.vec[p.id]);
            par_pos.vec[p.id] = p.pos;
            par_vel.vec[p.id] = p.vel;
        }
    }
}

pub fn render_particle_sim(
    mut query: Query<(&Particle, &mut Transform)>,
    par_pos: Res<ParticlePositions>,
    ) {
    for (p, mut t) in query.iter_mut() {
        t.translation = par_pos.vec[p.id];
    }
}

// TODO: cambiar esto para las celdas
pub fn shoot_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kb: Res<Input<KeyCode>>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vel: ResMut<ParticleVelocities>,
    cam_pos: Query<&Transform, With<FlyCam>>,
) {
    if kb.pressed(KeyCode::F) {
        let i = par_pos.vec.len();

        if let Ok(cam) = cam_pos.get_single() {
            let local_z = cam.local_z();

            let init_vel = local_z - local_z * 100.;

            let p = Particle::new(i, 1, false, cam.translation, init_vel, Attraction::default(), Repulsion::default());
            par_pos.vec.push(p.pos);
            par_vel.vec.push(p.vel);

            commands
                .spawn(PbrBundle {
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

// TODO: ccambiar esto para las celdas
pub fn spawn_locked_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vel: ResMut<ParticleVelocities>,
    kb: Res<Input<KeyCode>>,
    cam_pos: Query<&Transform, With<FlyCam>>,
) {

    if kb.just_pressed(KeyCode::B) {
        let i = par_pos.vec.len();

        if let Ok(cam) = cam_pos.get_single() {

            let p = Particle::new(i, 2, true, cam.translation, Vec3::ZERO, Attraction::new(0., 0.), Repulsion::new(P_RAD+1., 200.));
            par_pos.vec.push(p.pos);
            par_vel.vec.push(p.vel);

            commands
                .spawn(PbrBundle {
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

