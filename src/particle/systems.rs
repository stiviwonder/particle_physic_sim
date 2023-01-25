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
    mut chunk: ResMut<Chunk>,
    par_pos:  Res<ParticlePositions>,
    par_vel: Res<ParticleVelocities>,
) {
    for mut cell in chunk.cells.iter_mut() {
        for mut p in cell.parvec.iter_mut() {
//            println!("PID {}, pos {}\t par_pos = {}", p.id, p.pos, par_pos.vec[p.id]);
            p.pos = par_pos.vec[p.id];
            p.vel = par_vel.vec[p.id];
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

