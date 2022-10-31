use bevy::prelude::*;
use crate::particle::entities::Particle;

pub struct Chunk {
    pub cells: Vec<Cell>,
}
impl Chunk {
    pub fn default() -> Self {
        return Chunk {
            cells: Vec::new(),
        };
    }
}

pub struct Cell {
    pub id: usize,
    pub dim: Vec3,
    pub density: usize,
    pub parvec: Vec<Particle>, 
}

// NOTE: implementar aqui las nuevas posiciones??
impl Cell {
    pub fn new(i: usize, d: Vec3, pv: Vec<Particle>) -> Self {
        return Cell {
            id: i,
            dim: d,
            density: pv.len(),
            parvec: pv,
        };
    }
}
