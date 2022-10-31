use super::entities::*;
use bevy::prelude::*;
use crate::particle::entities::*;
use crate::particle::consts::*;

pub fn startup_chunk(
    mut commands: Commands,
    ) {

    const CHUNK_SIZE: usize = 10;
    let cell_dim = Vec3::ONE * 10.;
    let mut chunk = Chunk::default();
    let mut init_pos = Vec3::ZERO;

    for i in 0..CHUNK_SIZE {
        let mut parvec: Vec<Particle> = Vec::new();

        // FIXME: no funciona bien lo del offsize :(
        for j in 0..CHUNK_SIZE {
            let offsize = 1.;
            let p = Particle::new(
                j+i,
                0,
                false, 
                Vec3::new(
                    init_pos.x + offsize,
                    init_pos.y,
                    init_pos.z,
                    ),
                Vec3::ZERO,
                Attraction::default(), 
                Repulsion::default()
            );
            parvec.push(p);
        }

        let c = Cell::new(i, cell_dim, parvec);
        chunk.cells.push(c);

        if i % 3 == 0 {
            init_pos.x = 0.;
            init_pos.z += 10.
        } else {
            init_pos.x += 10.;
        }
    }

    commands.insert_resource(chunk);
    println!("Cells created");
}

pub fn spawn_particles (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk: Res<Chunk>,
) {
    let mut par_pos = ParticlePositions { vec: Vec::new() };
    let mut par_vels = ParticleVelocities { vec: Vec::new() };

    for cell in chunk.cells.iter() {
        for p in cell.parvec.iter() {
            par_pos.vec.push(p.pos);
            par_vels.vec.push(p.vel);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: p.radius,
                        subdivisions: SUBDIV,
                    })),
                    material: materials.add(Color::CYAN.into()),
                    transform: Transform::from_xyz(p.pos.x, p.pos.y, p.pos.z),
                    ..default()
                })
            .insert(*p);
        }
    }
    commands.insert_resource(par_pos);
    commands.insert_resource(par_vels);
}

pub fn print_cells(
    chunk: Res<Chunk>,
    ) {
    for cell in chunk.cells.iter() {
        println!("cell id: {}", cell.id);
        println!("cell density: {}", cell.density);
    }
}

