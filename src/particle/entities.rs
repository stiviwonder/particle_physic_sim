use bevy::prelude::*;

use super::consts::*;

// #[derive(Reflect, Component, Default)]
#[derive(Component, Default, Clone, Copy)]
pub struct Particle {
    pub id: usize,
    pub grp: usize,
    pub locked: bool,
    pub pos: Vec3,
    pub radius: f32,
    pub mass: f32,
    pub vel: Vec3,
    pub atr: Attraction,
    pub rep: Repulsion,
}

impl Particle {
    pub fn new(i: usize, g: usize, l: bool, p: Vec3, v: Vec3, a: Attraction, r: Repulsion) -> Self {
        return Particle {
            id: i,
            grp: g,
            locked: l,
            pos: p,
            radius: P_RAD,
            mass: P_MASS,
            vel: v,
            atr: a,
            rep: r,
        };
    }

    pub fn get_accel(&self, force: f32) -> f32 {
        return force / self.mass;
    }

    pub fn get_distance(&self, p2: &Particle) -> f32 {
        //          p - center_p
        let x = (self.pos.x - p2.pos.x) * (self.pos.x - p2.pos.x);
        let y = (self.pos.y - p2.pos.y) * (self.pos.y - p2.pos.y);
        let z = (self.pos.z - p2.pos.z) * (self.pos.z - p2.pos.z);

        return (x + y + z).sqrt();
    }

    pub fn on_floor(&self, floor: f32) -> bool {
        return self.pos.y <= floor;
    }

    pub fn attract(&self, p: &Particle) -> Vec3 {
        let dir = self.pos - p.pos;
        return dir.normalize_or_zero() * p.get_accel(self.atr.force);
    }

    pub fn repulse(&self, p: &Particle) -> Vec3 {
        let dir = -1. * (self.pos - p.pos);
        return dir.normalize_or_zero() * p.get_accel(self.rep.force);
    }
}

#[derive(Default, Clone, Copy)]
pub struct Attraction {
    pub radius: f32,
    pub force: f32,
}

impl Attraction {
    pub fn new(r: f32, f: f32) -> Self {
        return Attraction {
            radius: r,
            force: f,
        };
    }
    pub fn default() -> Self {
        return Attraction { radius: P_RAD+R_ATR, force: F_ATR };
    }
}

#[derive(Default, Clone, Copy)]
pub struct Repulsion {
    pub radius: f32,
    pub force: f32,
}
impl Repulsion {
    pub fn new(r: f32, f: f32) -> Self {
        return Repulsion {
            radius: r,
            force: f,
        };
    }
    pub fn default() -> Self {
        return Repulsion { radius: P_RAD+R_REP, force: F_REP };
    }
}

#[derive(Resource)]
pub struct ParticlePositions {
    pub vec: Vec<Vec3>,
}

#[derive(Resource)]
pub struct ParticleVelocities {
    pub vec: Vec<Vec3>,
}
