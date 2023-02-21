use bevy::prelude::*;

pub mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
//    pub const CHUNK_SIZE: usize = 1000;
//    pub const CHUNK_DIM: usize = 10;
    pub const CHUNK_SIZE: usize = 216;
    pub const CHUNK_DIM: usize = 6;
//    pub const CHUNK_SIZE: usize = 27;
//    pub const CHUNK_DIM: usize = 3;

    pub const CELL_DIM_X: f32 = 30.;
    pub const CELL_DIM_Y: f32 = 20.;
    pub const CELL_DIM_Z: f32 = 30.;

    pub const GROUPS: bool = false;
}
/* ===================== */

pub struct EnviromentPlugin;

impl Plugin for EnviromentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup, 
                systems::startup_chunk
                )
            .add_startup_system_to_stage(
                StartupStage::Startup, 
                systems::get_cell_neighbours
                )
            .add_startup_system_to_stage(
                StartupStage::Startup, 
                systems::spawn_particles
                )
            .add_system(systems::update_chunk)
            .add_system(systems::update_cell_state) 
            .add_system(systems::print_cells)
            ;
    }

}
