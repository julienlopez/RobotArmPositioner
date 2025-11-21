use crate::logic::infra::Infra;
use crate::logic::State;

use rand::Rng;

pub fn forward_kinematics(infra: &Infra, state: &State) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut cumulative_angle: f32 = 0.0;

    for (arm, angle) in infra.arms.iter().zip(state.angles.iter()) {
        cumulative_angle += angle;
        x += arm.length * cumulative_angle.cos();
        y += arm.length * cumulative_angle.sin();
    }

    (x, y)
}

pub fn random_angles(num_arms: usize) -> Vec<f32> {
    let mut rng = rand::rng();
    (0..num_arms)
        .map(|_| rng.random_range(-std::f32::consts::PI..std::f32::consts::PI))
        .collect()
}
