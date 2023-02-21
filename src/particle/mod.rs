use bevy::prelude::*;
//use std::rand::Rng;

pub mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
//    pub const NUM_PAR: usize = 27*5; 
    pub const NUM_PAR: usize = 216*4; 
//    pub const NUM_PAR: usize = 1000*3; 

    pub const SUBDIV: usize = 1;
    pub const P_RAD: f32 = 1.;
    pub const P_MASS: f32 = 5.5;

    pub const R_ATR: f32 = 15.5;
    pub const R_REP: f32 = 15.;
    pub const F_ATR: f32 = 57.5;
    pub const F_REP: f32 = 67.;

    pub const AIR_F: f32 = 0.90;
    pub const GRAVITY: f32 = 9.8;
    pub const FLOOR_F: f32 = 0.49;

    pub const LOCKED: bool = false;
}
/* ===================== */

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(systems::sync_particle_data)
            .add_system(systems::render_particle_sim)
            ;
    }
}
