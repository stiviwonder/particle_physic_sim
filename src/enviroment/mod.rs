use bevy::prelude::*;

pub mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
    pub const CHUNK_SIZE: usize = 125;
    pub const CHUNK_DIM: usize = 5;

    pub const CELL_DIM_X: f32 = 50.;
    pub const CELL_DIM_Y: f32 = 20.;
    pub const CELL_DIM_Z: f32 = 50.;
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
