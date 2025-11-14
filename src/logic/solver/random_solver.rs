use crate::logic::infra::Infra;
use dioxus::logger::tracing;
use rand::Rng;
use crate::logic::solver::solver::Solver;

pub struct RandomSolver;

impl Solver for RandomSolver {
fn solve(infra: &Infra, target: (f32, f32)) -> Option<Vec<f32>> {
    tracing::info!("Solving for target: {:?}", target);

    let res = (0.. 5000).map(|_| {
        let angles = random_angles(infra.arms.len());
        let (end_x, end_y) = super::utils::forward_kinematics(infra, &angles);
        // tracing::info!("Trying: {angles:?} -> ({end_x}, {end_y})");
        let dist_sq = (end_x - target.0).powi(2) + (end_y - target.1).powi(2);
        (dist_sq, angles)
    }).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // tracing::info!("Solver result: {:?}", res);

    res.map(|(_, angles)| angles)
}

}

fn random_angles(num_arms: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..num_arms)
        .map(|_| rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI))
        .collect()
}
