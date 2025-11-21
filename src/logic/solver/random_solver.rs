use crate::logic::infra::Infra;
use crate::logic::solver::solver::Solver;
use crate::logic::State;
use dioxus::logger::tracing;

pub struct RandomSolver;

impl Solver for RandomSolver {
    fn solve(infra: &Infra, target: (f32, f32)) -> Option<State> {
        tracing::info!("Solving for target: {:?}", target);

        let res = (0..5000)
            .map(|_| {
                let state = State {
                    angles: super::utils::random_angles(infra.arms.len()),
                };
                let (end_x, end_y) = super::utils::forward_kinematics(infra, &state);
                // tracing::info!("Trying: {state:?} -> ({end_x}, {end_y})");
                let dist_sq = (end_x - target.0).powi(2) + (end_y - target.1).powi(2);
                (dist_sq, state)
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // tracing::info!("Solver result: {:?}", res);

        res.map(|(_, state)| state)
    }
}
