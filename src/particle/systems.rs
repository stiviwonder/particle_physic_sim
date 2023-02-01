use crate::enviroment::entities::Chunk;

use super::entities::*;
use bevy::prelude::*;
// use bevy_flycam::FlyCam;

use super::consts::*;



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

