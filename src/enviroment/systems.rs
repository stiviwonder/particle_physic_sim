use super::entities::*;
use bevy::prelude::*;
use crate::particle::entities::*;
use crate::particle::consts::*;
use rand::Rng;

use super::consts::*;

// NOTE: revisar si el pid va bien
fn new_parvec(init_pos: Vec3, cell_dim: Vec3, pid: &mut usize, gid: usize) -> Vec<Particle> {
    let mut parvec: Vec<Particle> = Vec::new();
    let mut offsize = Vec3::ZERO;

    // TODO: NUMPAR full loading on cells
    for _ in 0..NUM_PAR/CHUNK_SIZE {
        let rand_offset: Vec3 = Vec3::new(rand::thread_rng().gen_range(0.0..2.0),
                                          rand::thread_rng().gen_range(0.0..5.0),
                                          rand::thread_rng().gen_range(0.0..2.0));

        let p = Particle::new(
            *pid, 
            gid,
            LOCKED, 
            fit_in_chunk(init_pos+offsize+rand_offset),
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
    let cell_dim = Vec3::new(CELL_DIM_X, CELL_DIM_Y, CELL_DIM_Z);

    let mut init_pos = Vec3::ZERO;

    let mut pid: usize = 0;
    let mut gid: usize = 0;

    for i in 0..CHUNK_SIZE {

        let parvec = new_parvec(init_pos, cell_dim, &mut pid, gid);
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

        if GROUPS == true {
            gid += 1;
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

// TODO: Mover al modulo de particulas
pub fn spawn_particles (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk: Res<Chunk>,
) {
    let mut par_pos = ParticlePositions  { vec: [Vec3::ZERO; NUM_PAR]};
    let mut par_vel = ParticleVelocities { vec: [Vec3::ZERO; NUM_PAR]};

    for cell in chunk.cells.iter() {
        let color = if GROUPS == true {
            random_color()
        } else {
            Color::CYAN
        };
        for p in cell.parvec.iter() {
            par_pos.vec[p.id] = p.pos;
            par_vel.vec[p.id] = p.vel;
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
    commands.insert_resource(par_vel);
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
                chunk.cells[index(x, y, z)].neigh[nidx] = index(nx, ny, nz) as isize;
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

fn fit_in_chunk(pos: Vec3) -> Vec3 {
    let mut new_pos: Vec3 = Vec3::ZERO;

    if pos.x < 0. {
        new_pos.x = 0.;
    } else if pos.x > CHUNK_DIM as f32 * CELL_DIM_X {
        new_pos.x = CHUNK_DIM as f32 * CELL_DIM_X;
    } else {
        new_pos.x = pos.x;
    }

    if pos.y < 0. {
        new_pos.y = 0.;
    } else if pos.y > CHUNK_DIM as f32 * CELL_DIM_Y {
        new_pos.y = CHUNK_DIM as f32 * CELL_DIM_Y;
    } else {
        new_pos.y = pos.y;
    }

    if pos.z < 0. {
        new_pos.z = 0.;
    } else if pos.z > CHUNK_DIM as f32 * CELL_DIM_Z {
        new_pos.z = CHUNK_DIM as f32 * CELL_DIM_Z;
    } else {
        new_pos.z = pos.z;
    }

    return new_pos;
}

fn reject_vel(par: &Particle, vel: Vec3) -> Vec3 {
    let chunk: f32 = CHUNK_DIM as f32;
    let borders: [f32;6] = [0., 0., 0., CELL_DIM_X*chunk, CELL_DIM_Y*chunk, CELL_DIM_Z*chunk,];
    let normals: [Vec3;6] = [Vec3::X, Vec3::Y, Vec3::Z, Vec3::NEG_X, Vec3::NEG_Y, Vec3::NEG_Z,];
    let mut new_vel = vel;

    for i in 0..5 {
        if par.on_border(borders[i], i) {
            let nvec: Vec3 = normals[i];
            new_vel = (new_vel - (2. * (new_vel.dot_into_vec(nvec)) * nvec)) * FLOOR_F;
        }
    }

    return new_vel;
}

pub fn update_chunk (
    time: Res<Time>,
    mut chunk: ResMut<Chunk>,
    mut par_pos:  ResMut<ParticlePositions>,
    mut par_vel: ResMut<ParticleVelocities>,
    ) {

    // FIXME: si p1 locked ni te molestes en calcular nada
    for cell in chunk.cells.iter() {
        for p1 in cell.parvec.iter() {
            if p1.locked {
                continue;
            }

            let mut new_vel = p1.vel;

            // NOTE: si te flipas puedes limitar el check con
            //       cell_dim y R_ATTR y asi solo mirar las 
            //       celdas que entren en el radio
            for n in cell.neigh.iter().filter(|&&x| x != -1) {
                let idx: usize = *n as usize;

                for p2 in chunk.cells[idx].parvec.iter() {
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
            }

            // gravity
            new_vel -= Vec3::Y * GRAVITY;

            // rejection vector in case of a boundary collision
            new_vel = reject_vel(p1, new_vel);
//            if p1.pos.y == 0. {
//                new_vel.y = new_vel.y.abs() * FLOOR_F;
//            }

            new_vel *= AIR_F;

            let new_pos = p1.pos + new_vel * time.delta_seconds();

            par_pos.vec[p1.id] = fit_in_chunk(new_pos);
            par_vel.vec[p1.id] = new_vel;
            
        }
    }
}

fn is_inside_cell(pos: Vec3, c: &Cell) -> bool {
    return  pos.x >= c.pos.x &&
        pos.x <= c.pos.x + c.dim.x &&
        pos.y >= c.pos.y &&
        pos.y <= c.pos.y + c.dim.y &&
        pos.z >= c.pos.z &&
        pos.z <= c.pos.z + c.dim.z;
}

// NOTE: se supone que la nueva pos no tendra un salto tan grande 
//       como para saltarse una celda entera
fn new_host_idx(pos: Vec3, c: &Cell) -> usize {
    // --- LOW PLANE --- //
    // 1 st row
    if  pos.x < c.pos.x &&
        pos.y < c.pos.y &&
        pos.z < c.pos.z {
            return 0;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z < c.pos.z {
            return 1;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z < c.pos.z {
            return 2;
        }
    // 2 nd row
    else if  pos.x < c.pos.x &&
        pos.y < c.pos.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 3;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 4;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 5;
        }
    // 3 rd row
    else if  pos.x < c.pos.x &&
        pos.y < c.pos.y &&
        pos.z > c.pos.z + c.dim.z {
            return 6;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z > c.pos.z + c.dim.z {
            return 7;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y < c.pos.y &&
        pos.z > c.pos.z + c.dim.z {
            return 8;
        }

    // --- MID PLANE --- //
    // 1 st row
    else if  pos.x < c.pos.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 9;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 10;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 11;
        }
    // 2 nd row
    else if  pos.x < c.pos.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 12;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 14;
        }
    // 3 rd row
    else if  pos.x < c.pos.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 15;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 16;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y >= c.pos.y && pos.y <= c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 17;
        }

    // --- HIGH PLANE --- //
    // 1 st row
    else if  pos.x < c.pos.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 18;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 19;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z < c.pos.z {
            return 20;
        }
    // 2 nd row
    else if  pos.x < c.pos.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 21;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 22;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z >= c.pos.z && pos.z <= c.pos.z + c.dim.z {
            return 23;
        }
    // 3 rd row
    else if  pos.x < c.pos.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 24;
        }
    else if  pos.x >= c.pos.x && pos.x <= c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 25;
        }
    else if  pos.x > c.pos.x + c.dim.x &&
        pos.y > c.pos.y + c.dim.y &&
        pos.z > c.pos.z + c.dim.z {
            return 26;
        }

    return 13; // index of the actual host cell
}

// FIXME: Borra los punto clone >:(
// FIXME: esto peta porque la newpos se mete en -1 solucion en update_chunk
pub fn update_cell_state(
    mut chunk: ResMut<Chunk>,
    par_pos:  Res<ParticlePositions>,
    ) {

    let cells = chunk.cells.clone(); // esto me gusta muy muy poco
                                     //
    let mut ci: usize = 0;
    for cell in cells.iter() {
        let mut pi: usize = 0; // paticle postion inside cell vector
         
        let parvec = cell.parvec.clone(); // ke poko me gusta esto >:(
         
        for p in parvec.iter() {
            if !is_inside_cell(par_pos.vec[p.id], cell) {
                let n_idx: usize = new_host_idx(par_pos.vec[p.id], cell);
                
                // suponemos que no dejamos que vaya a ningun -1
                let idx: usize = cell.neigh[n_idx] as usize;

                chunk.cells[idx].parvec.push(*p);
                chunk.cells[ci].parvec.remove(pi);
            } else {
                pi += 1;
            }
        }
        ci += 1;
    }
}

pub fn print_cells(
    chunk: Res<Chunk>,
    kb: Res<Input<KeyCode>>
    ) {
//    if kb.pressed(KeyCode::F1) {
    if kb.just_pressed(KeyCode::F1) {
        for cell in chunk.cells.iter() {
        let cell = &chunk.cells[0];
            println!("cell id: {}", cell.id);
            println!("cell density: {}", cell.density);
            println!("cell pos: {}", cell.pos);
            cell.print_particles();
            println!("========================================");
            println!("========================================");
            
        }
    }
}

