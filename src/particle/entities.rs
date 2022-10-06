use bevy::prelude::*;

// #[derive(Reflect, Component, Default)]
#[derive(Component)]
pub struct Particle {
    pub id: usize,
    pub grp: usize,
    pub pos: Vec3,
    pub radius: f32,
    pub mass: f32,
    pub vel: Vec3,
    pub att: Attraction,
    pub rep: Repulsion,
}

impl Particle {
    pub fn get_accel(&self, force: f32) -> f32 {
        return force / self.mass;
    }

    pub fn on_floor(&self) -> bool {
        return self.pos.y <= 0.0
    }

    pub fn attract(&self, p: &Particle) -> Vec3 {
        let dir = self.pos - p.pos;
        return dir.normalize_or_zero() * p.get_accel(self.att.force);
    }

    pub fn repulse(&self, p: &Particle) -> Vec3 {
        let dir = -1. * (self.pos - p.pos);
        return dir.normalize_or_zero() * p.get_accel(self.rep.force);
    }
}

// #[derive(Default)]
pub struct Attraction {
    pub radius: f32,
    pub force: f32,
}

// #[derive(Default)]
pub struct Repulsion {
    pub radius: f32,
    pub force: f32,
}

pub struct ParticlePositions {
    pub vec: Vec<Vec3>,
}

pub struct ParticleVelocities {
    pub vec: Vec<Vec3>,
}
