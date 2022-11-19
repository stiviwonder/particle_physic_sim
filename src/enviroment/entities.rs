use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use crate::particle::entities::Particle;
use crate::particle::consts::*;

use super::consts::*;

#[derive(Resource)]
pub struct Chunk {
    pub cells: [Arc<Mutex<Cell>>; CHUNK_SIZE],
}
impl Chunk {
    pub fn default() -> Self {
        return Chunk {
            cells: [Arc::new(Mutex::new(Cell::EMPTY)); CHUNK_SIZE],
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
    pub arc_neigh: Vec<Arc<Mutex<Cell>>>,
}

// NOTE: implementar aqui las nuevas posiciones??
impl Cell {
    pub const EMPTY: Self = Cell {
            id: 0,
            pos: Vec3::ZERO,
            dim: Vec3::ZERO,
            density: 0,
            parvec: Vec::new().into(),
            arc_neigh: Vec::new().into(),
    };

    pub fn new(i: usize, p: Vec3, d: Vec3, pv: Vec<Particle>) -> Self {
        return Cell {
            id: i,
            pos: p,
            dim: d,
            density: pv.len(),
            parvec: pv,
            arc_neigh: Vec::new(),
        };
    }

    pub fn update(&mut self, time: Time) {
        let mut pos_v: Vec<Vec3> = Vec::new();

        for p1 in self.parvec.iter() {
            let mut new_vel = p1.vel;

            for p2 in self.parvec.iter() {
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
                //add_gravity(&mut new_vel);

                if p1.on_floor(self.pos.y) {
                    new_vel.y = new_vel.y.abs() * FLOOR_F;
                }

                new_vel *= AIR_F;

                let new_pos = p1.pos + new_vel * time.delta_seconds();
                pos_v.push(new_pos);
            }
        }

        let mut i: usize = 0;
        for mut p in self.parvec.iter_mut() {
            p.pos = pos_v[i];
            i += 1;
        }
    }


}
