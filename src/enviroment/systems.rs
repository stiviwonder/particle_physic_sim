
use super::entities::*;
use bevy::prelude::*;
use crate::particle::entities::*;
use crate::particle::consts::*;
use rand::Rng;

use super::consts::*;

// NOTE: revisar si el pid va bien
fn new_parvec(init_pos: Vec3, cell_dim: Vec3, pid: &mut usize) -> Vec<Particle> {
    let mut parvec: Vec<Particle> = Vec::new();
    let mut offsize = Vec3::ZERO;

    // TODO: NUMPAR full loading on cells
    for _ in 0..NUM_PAR/CHUNK_SIZE {
        let rand_offset: Vec3 = Vec3::new(rand::thread_rng().gen_range(0.0..2.0),
                                          rand::thread_rng().gen_range(0.0..5.0),
                                          rand::thread_rng().gen_range(0.0..2.0));

        let p = Particle::new(
            *pid, 
            0,
            LOCKED, 
            init_pos+offsize+rand_offset,
            Vec3::ZERO,
            Attraction::default(), 
            Repulsion::default()
            );
        parvec.push(p);
        println!("pid {}, pos {}", pid, p.pos);

        // Adjust the offfsize to fill de cell
        offsize.x = (offsize.x +1.) % cell_dim.x;
        if offsize.x % cell_dim.x == 0. { 
            offsize.z = (offsize.z+1.) % cell_dim.z;
        };
        if offsize.z % cell_dim.z == 0. && offsize.x % cell_dim.x == 0.  { 
            offsize.y = (offsize.y+1.) % cell_dim.y;
        };

        *pid += 1;
    }

    return parvec;
}

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

        let parvec = new_parvec(init_pos, cell_dim, &mut pid);
        let c = Cell::new(i, init_pos, cell_dim, parvec);
//        chunk.cells[i] = Arc::new(c);
        chunk.cells[i] = c;

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

pub fn random_color() -> Color {
        let r: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let g: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let b: f32 = rand::thread_rng().gen_range(0.0..1.0);
        return Color::rgb(r, g, b);
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
        let color = random_color();

        for p in cell.parvec.iter() {
            par_pos.vec.push(p.pos);
            par_vels.vec.push(p.vel);
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: p.radius,
                        subdivisions: SUBDIV,
                    })),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(p.pos.x, p.pos.y, p.pos.z),
                    ..default()
                })
            .insert(*p);
        }
    }
    commands.insert_resource(par_pos);
    commands.insert_resource(par_vels);
}

fn index (x: isize, y: isize, z: isize) -> usize{
    let dim: isize = CHUNK_DIM as isize;
    return (z + (x * dim) + (y*dim*dim)) as usize;
}

// TODO: arreglar el lio del isize y optimizar los algorismos
 pub fn get_cell_neighbours(
     mut chunk: ResMut<Chunk>,
     ) {
    for y in 0..CHUNK_DIM as isize{
    for x in 0..CHUNK_DIM as isize{
    for z in 0..CHUNK_DIM as isize{
        let mut nidx: usize = 0;

        //                 println!("\n\ncell {},{},{} -> ", x,y,z);
        for off_y in -1..2 as isize{
        for off_x in -1..2 as isize{
        for off_z in -1..2 as isize{
            let in_x: bool = x+off_x >= 0 && x+off_x <= (CHUNK_DIM-1) as isize;
            let in_y: bool = y+off_y >= 0 && y+off_y <= (CHUNK_DIM-1) as isize;
            let in_z: bool = z+off_z >= 0 && z+off_z <= (CHUNK_DIM-1) as isize;

            if in_x && in_y && in_z {
                let nx: isize = x + off_x;
                let ny: isize = y + off_y;
                let nz: isize = z + off_z;

                //                                 print!("({},{},{}, i{})\t", nx,ny,nz, index(nx, ny, nz));
                //                                 print!("({},{},{}, nid{})\t", nx,ny,nz, nidx);
                if index(x, y, z) != index(nx, ny, nz) {
                    chunk.cells[index(x, y, z)].neigh[nidx] = index(nx, ny, nz) as isize;
                }
            }
            nidx += 1;

        }
        }
        }

    }
    }
    }
//    for c in chunk.cells.iter() {
//        c.print_neighbours();
//    }
 }

// FIXME: aqui esta el fallo
pub fn update_chunk (
    time: Res<Time>,
    mut chunk: ResMut<Chunk>,
    ) {

    let mut pos_v: Vec<Vec3> = Vec::new();
    for cell in chunk.cells.iter() {
        for p1 in cell.parvec.iter() {
            let mut new_vel = p1.vel;

            for p2 in cell.parvec.iter() {
                if p1.id != p2.id && !p1.locked {
                    let dis = p1.get_distance(p2);

                    if dis <= p2.rep.radius {
                        new_vel += p2.repulse(p1);
                    } else if dis <= p2.atr.radius && 
                              p1.grp == p2.grp {

                        new_vel += p2.attract(p1);
                    }
                }
            }

            if !p1.locked {

                // gravity
                new_vel -= Vec3::Y * GRAVITY;

                if p1.on_floor(cell.pos.y) {
                    new_vel.y = new_vel.y.abs() * FLOOR_F;
                }

                new_vel *= AIR_F;

                let new_pos = p1.pos + new_vel * time.delta_seconds();
                pos_v.push(new_pos);
            }
        }
    }

    for mut cell in chunk.cells.iter_mut() {
        for mut p in cell.parvec.iter_mut() {
            p.pos = pos_v[p.id];
        }
    }
}

pub fn print_cells(
    chunk: Res<Chunk>,
    kb: Res<Input<KeyCode>>,
    ) {
    if kb.pressed(KeyCode::F1) {
//    if kb.just_pressed(KeyCode::F1) {
//        for cell in chunk.cells.iter() {
        let cell = &chunk.cells[0];
            println!("cell id: {}", cell.id);
            println!("cell density: {}", cell.density);
            println!("cell pos: {}", cell.pos);
            cell.print_particles();
            println!("========================================");
            println!("========================================");
            
        // }
    }
}

