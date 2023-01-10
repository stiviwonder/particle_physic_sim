use bevy::prelude::*;

pub mod entities;
mod systems;

/* ===== Constants ===== */
pub mod consts {
    pub const CHUNK_SIZE: usize = 27;
    pub const CHUNK_DIM: usize = 3;
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
//            .add_startup_system_to_stage(
//                StartupStage::Startup, 
//                systems::debug_cube_cell_spawn
//                )
            .add_system(systems::update_chunk)
            .add_system(systems::print_cells)
            ;
    }

}
