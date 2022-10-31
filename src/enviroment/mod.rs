use bevy::prelude::*;

pub mod entities;
mod systems;

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
                systems::spawn_particles
                )
            .add_system(systems::print_cells)
            ;
    }

}
