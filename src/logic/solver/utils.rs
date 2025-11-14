use crate::logic::infra::Infra;

pub fn forward_kinematics(infra: &Infra, angles: &Vec<f32>) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut cumulative_angle: f32 = 0.0;

    for (arm, angle) in infra.arms.iter().zip(angles.iter()) {
        cumulative_angle += angle;
        x += arm.length * cumulative_angle.cos();
        y += arm.length * cumulative_angle.sin();
    }

    (x, y)
}

