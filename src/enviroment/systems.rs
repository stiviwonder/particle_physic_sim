use std::sync::Arc;

use super::entities::*;
use bevy::prelude::*;
use crate::particle::entities::*;
use crate::particle::consts::*;

use super::consts::*;

// TODO: CHUNK_DIM generico con un cuberoot
//       A lo mejor meter en una fn lo de ajustar la pos
pub fn startup_chunk(
    mut commands: Commands,
    ) {

    let mut chunk = Chunk::default();
    let cell_dim = Vec3::ONE * 10.;

    let mut init_pos = Vec3::ZERO;

    let mut pid: usize = 0;

    for i in 0..CHUNK_SIZE {
        let mut parvec: Vec<Particle> = Vec::new();
        let mut offsize = Vec3::ZERO;

        // TODO: NUMPAR full loading on cells
        for _ in 0..NUM_PAR/CHUNK_SIZE {
            let p = Particle::new(
                pid, 
                0,
                LOCKED, 
                init_pos+offsize,
                Vec3::ZERO,
                Attraction::default(), 
                Repulsion::default()
            );
            parvec.push(p);
            println!("pid {}", pid);

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

        let c = Cell::new(i, init_pos, cell_dim, parvec);
        chunk.cells[i] = Arc::new(c);

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

fn index (x: usize, y: usize, z: usize) -> usize{
    return x + (y * CHUNK_DIM) + (z*CHUNK_DIM*CHUNK_DIM);
}

 pub fn get_cell_neighbours(
     chunk: Res<Chunk>,
     ) {
      let mut arc_net: Vec<Arc<Cell>> = Vec::new();
      let mut v_net: Vec<Vec<Arc<Cell>>> = Vec::new();
 
      for x in 0..CHUNK_DIM {
          for y in 0..CHUNK_DIM {
              for z in 0..CHUNK_DIM {
                  let arc_n0 = Arc::clone(&chunk.cells[index(x, y, z)]);
                  v_net.push(arc_net)
              }
          }
      }
 }

pub fn debug_cube_cell_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk: Res<Chunk>,
    ) {
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    for cell in chunk.cells.iter() {
        let pos = cell.parvec.first().unwrap().pos;
        println!("cid {} pos {}", cell.id, pos);
        let offset = cell.dim.x / 2.;
        commands.spawn_bundle(PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Cube {size: cell.dim.x})),
            material: materials.add(Color::rgb_u8(r, g, b).into()),
            transform: Transform::from_xyz(pos.x+offset, pos.y+offset, pos.z+offset),
            ..default()
        });

        r += 1;
        g += 5;
        b += 3;

    }
}

pub fn print_cells(
    chunk: Res<Chunk>,
    kb: Res<Input<KeyCode>>,
    ) {
    if kb.pressed(KeyCode::F1) {
        for cell in chunk.cells.iter() {
            println!("cell id: {}", cell.id);
            println!("cell density: {}", cell.density);
            println!("cell pos: {}", cell.parvec.first().unwrap().pos);
            
        }
    }
}

