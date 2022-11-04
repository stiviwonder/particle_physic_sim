use super::entities::*;
use bevy::prelude::*;
use crate::particle::entities::*;
use crate::particle::consts::*;

// TODO: CHUNK_DIM generico con un cuberoot
//       A lo mejor meter en una fn lo de ajustar la pos
pub fn startup_chunk(
    mut commands: Commands,
    ) {

    const CHUNK_SIZE: usize = 8;
    const CHUNK_DIM: usize = 2;
    let cell_dim = Vec3::ONE * 10.;
    let mut chunk = Chunk::default();
    let mut init_pos = Vec3::ZERO;

    for i in 0..CHUNK_SIZE {
        let mut parvec: Vec<Particle> = Vec::new();
        let mut offsize = Vec3::ZERO;
        let mut pid: usize = 0;

        for _ in 0..NUM_PAR/CHUNK_SIZE {
            let p = Particle::new(
                pid, 
                0,
                false, 
                init_pos+offsize,
                Vec3::ZERO,
                Attraction::default(), 
                Repulsion::default()
            );
            parvec.push(p);

            // Adjust the offfsize to fill de cell
            offsize.x = (offsize.x +1.) % cell_dim.x;
            if offsize.x % cell_dim.x == 0. { 
                offsize.z = (offsize.z+1.) % cell_dim.z;
            };
            if offsize.z % cell_dim.z == 0. && offsize.x % cell_dim.x == 0.  { 
                offsize.y = (offsize.y+1.) % cell_dim.y;
            };

            pid += 1;
        }

        let c = Cell::new(i, cell_dim, parvec);
        chunk.cells.push(c);

        // Each cell starting position
        if (i+1) % CHUNK_DIM.pow(2) == 0 {
            init_pos.x = 0.;
            init_pos.y += cell_dim.y;
            init_pos.z = 0.;
        } else if (i+1) % CHUNK_DIM == 0 { 
            init_pos.x = 0.;
            init_pos.z += cell_dim.z;
        } else {
            init_pos.x += cell_dim.x;
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
    kb: Res<Input<KeyCode>>,
    ) {
    if kb.pressed(KeyCode::F1) {
        for cell in chunk.cells.iter() {
            println!("cell id: {}", cell.id);
            println!("cell density: {}", cell.density);
        }
    }
}

