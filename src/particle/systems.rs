// NOTE: una mierda para hacer dos cambiar para spawn generico de particulas
use bevy::prelude::*;
use super::entities::*;

use super::consts::*;

pub fn startup_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut offsize = 0.;
    let mut par_pos = ParticlePositions { vec: Vec::new() };
    let mut par_vels = ParticleVelocities { vec: Vec::new() };

    let mut init_pos = Vec3 {
        x: -10.,
        y: 5.0,
        z: -10.,
    };

    for i in 0..NUM_PAR {
        let p = Particle {
            id: i,
            pos: Vec3 {
                x: init_pos.x + offsize,
                y: init_pos.y + offsize,
                z: init_pos.z - offsize,
            },
            radius: P_RAD,
            mass: 1.,
            vel: Vec3::ZERO,
            attrac: Attraction {
                radius: P_RAD + R_ATR,
                force: F_ATR,
            },
            rep: Repulsion {
                radius: P_RAD + R_REP,
                force: F_REP,
            },
        };

        println!("Particle pos: {}", p.pos);
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
        .insert(p);

        if i % 10 == 0 {
            init_pos.z += 1.;
            offsize = 0.;
        } else {
            offsize += 5.0;
        }
    }

    commands.insert_resource(par_pos);
    commands.insert_resource(par_vels);
}

pub fn get_distance(p1: Vec3, p2: Vec3) -> f32 {
    //          p - center_p
    let x = (p1.x - p2.x) * (p1.x - p2.x);
    let y = (p1.y - p2.y) * (p1.y - p2.y);
    let z = (p1.z - p2.z) * (p1.z - p2.z);

    return (x + y + z).sqrt();
}

// TODO: modificar para cambiar el valor por referencias
pub fn add_gravity(vel: Vec3) -> Vec3 {

    return vel - Vec3::Y * GRAVITY;
}

pub fn get_new_pos(
    time: Res<Time>,
    particles: Query<&Particle>,
    mut par_pos: ResMut<ParticlePositions>,
    mut par_vels: ResMut<ParticleVelocities>,
) {
    let mut i = 0;
    let minus = Vec3::new(-1.0, -1.0, -1.0);

    for p1 in particles.iter() {
        let mut new_vel = p1.vel;

        for p2 in particles.iter() {
            if p1.id != p2.id {
                let dis = get_distance(p1.pos, p2.pos);

                if dis <= p2.rep.radius {
                    let dir = minus * (p2.pos - p1.pos);
                    new_vel += dir.normalize_or_zero() * p1.get_accel(p2.rep.force);
                } else if dis <= p2.attrac.radius {
                    let dir = p2.pos - p1.pos;
                    new_vel += dir.normalize_or_zero() * p1.get_accel(p2.attrac.force);
                }
            }
        }

        new_vel = add_gravity(new_vel);
        if p1.on_floor() {
            new_vel.y = new_vel.y.abs() * FLOOR_F;
        }

        new_vel *= AIR_F;
        let new_pos = p1.pos + new_vel * time.delta_seconds();
        par_pos.vec[i] = new_pos;
        par_vels.vec[i] = new_vel;
        i += 1;
    }
}

pub fn sync_par_data(
    mut particles: Query<&mut Particle>,
    par_pos: Res<ParticlePositions>,
    par_vels: Res<ParticleVelocities>,
) {
    let mut i = 0;
    for mut p in particles.iter_mut() {
        p.pos = par_pos.vec[i];
        p.vel = par_vels.vec[i];
        i += 1;
    }
}

pub fn render_particle_sim(mut query: Query<(&mut Transform, With<Particle>, &Particle)>) {
    for (mut t, _, p) in query.iter_mut() {
        t.translation = p.pos;
    }
}
