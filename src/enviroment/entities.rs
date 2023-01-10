use bevy::prelude::*;
use crate::particle::entities::Particle;
use crate::particle::consts::*;

use super::consts::*;

#[derive(Resource)]
pub struct Chunk {
    pub cells: [Cell; CHUNK_SIZE as usize],
}
impl Chunk {
    pub fn default() -> Self {
        return Chunk {
            cells: [Cell::EMPTY; CHUNK_SIZE],
        };
    }
}


#[derive(Default, Clone)]
pub struct Cell {
    pub id: usize,
    pub pos: Vec3,
    pub dim: Vec3,
    pub density: usize,
    pub parvec: Vec<Particle>, 
    pub neigh: [isize; 27],
}

// NOTE: implementar aqui las nuevas posiciones??
impl Cell {
    pub const EMPTY: Self = Cell {
            id: 0,
            pos: Vec3::ZERO,
            dim: Vec3::ZERO,
            density: 0,
            parvec: Vec::new(),
            neigh: [-1;27],
    };

    pub fn new(i: usize, p: Vec3, d: Vec3, pv: Vec<Particle>) -> Self {
        return Cell {
            id: i,
            pos: p,
            dim: d,
            density: pv.len(),
            parvec: pv,
            neigh: [-1;27],
        };
    }

    pub fn print_neighbours(&self) {
        println!("Cell {} neighbours -> ", self.id);
        let mut cont = 0;
        for n in self.neigh.iter() {
            if *n != -1 {
                print!("\x1b[93m {} \x1b[0m", n);
            } else {
                print!(" {} ", n);
            }

            cont += 1;
            if cont % CHUNK_DIM == 0 {
                print!("|");
            } 
            if cont == CHUNK_DIM*CHUNK_DIM {
                print!("|");
                cont = 0;
            }
        }
        println!();
    }

    pub fn print_particles(&self) {
        for p in self.parvec.iter() {
            p.print_debug();
        }
    }

}
