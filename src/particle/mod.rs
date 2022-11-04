use bevy::prelude::*;
//use std::rand::Rng;

pub mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
    pub const NUM_PAR: usize = 200; // NOTE: 8x25 

    pub const SUBDIV: usize = 1;
    pub const P_RAD: f32 = 1.;
    pub const P_MASS: f32 = 0.5;

    pub const R_ATR: f32 = 8.0;
    pub const R_REP: f32 = 5.;
    pub const F_ATR: f32 = 0.5;
    pub const F_REP: f32 = 1.;

    pub const AIR_F: f32 = 0.99;
    pub const GRAVITY: f32 = 1.0;
    pub const FLOOR_F: f32 = 0.89;
}
/* ===================== */

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
//            .add_startup_system(systems::startup_particles)
            .add_system(systems::shoot_particle)
            .add_system(systems::spawn_locked_particle)
            .add_system(systems::get_new_pos)
            .add_system(systems::sync_particle_data)
            .add_system(systems::render_particle_sim)
            //.register_type::<Particle>()
            ;
    }
}
