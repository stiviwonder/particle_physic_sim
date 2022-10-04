use bevy::prelude::*;

// #[derive(Reflect, Component, Default)]
#[derive(Component)]
pub struct Particle {
    pub id: usize,
    pub pos: Vec3,
    pub radius: f32,
    pub mass: f32,
    pub vel: Vec3,
    pub attrac: Attraction,
    pub rep: Repulsion,
}

impl Particle {
    pub fn get_accel(&self, force: f32) -> f32 {
        return force / self.mass;
    }

    pub fn on_floor(&self) -> bool {
        return self.pos.y <= 0.0
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
