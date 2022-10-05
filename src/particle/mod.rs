use bevy::prelude::*;
//use std::rand::Rng;

mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
    pub const NUM_PAR: usize = 20; 

    pub const SUBDIV: usize = 1;
    pub const P_RAD: f32 = 1.;

    pub const R_ATR: f32 = 16.0;
    pub const R_REP: f32 = 15.;
    pub const F_ATR: f32 = 0.5;
    pub const F_REP: f32 = 1.;

    pub const AIR_F: f32 = 0.99;
    pub const GRAVITY: f32 = 2.0;
    pub const FLOOR_F: f32 = 0.89;

    pub const WALL: f32 = 15.;
}
/* ===================== */

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(systems::startup_particles)
            .add_system(systems::get_new_pos)
            .add_system(systems::sync_par_data)
            .add_system(systems::render_particle_sim)
            //.register_type::<Particle>()
            ;
    }
}
