use crate::logic::infra::Infra;
use crate::logic::State;
// use dioxus::logger::tracing;
use crate::logic::solver::solver::Solver;

pub struct PsoSolver;

const W: f32 = 0.5;
const C1: f32 = 1.5;
const C2: f32 = 1.5;
const N: usize = 5;

impl Solver for PsoSolver {
    fn solve(infra: &Infra, target: (f32, f32)) -> Option<State> {
        let fitness = |angles: &State| {
            let (end_x, end_y) = super::utils::forward_kinematics(infra, angles);
            let dist_sq = (end_x - target.0).powi(2) + (end_y - target.1).powi(2);
            dist_sq
        };

        let mut x: Vec<State> = (0..N)
            .map(|_| State {
                angles: super::utils::random_angles(infra.arms.len()),
            })
            .collect();
        let mut v: Vec<State> = (0..N).map(|_| State::new(infra.arms.len())).collect();
        let mut g_best: Option<State> = None;
        let mut p_best = x.clone();

        for i in 0..N {
            let r1: f32 = rand::random();
            let r2: f32 = rand::random();

            v[i] = W * v[i].clone() + C1 * r1 * (p_best[i].clone() - x[i].clone());
            if let Some(g_best_val) = &g_best {
                v[i] += C2 * r2 * (g_best_val.clone() - x[i].clone());
            }
            x[i] += v[i].clone();

            if fitness(&x[i]) < fitness(&p_best[i]) {
                p_best = x.clone();
            }
            if fitness(&x[i]) < fitness(g_best.as_ref().unwrap()) {
                g_best = Some(x[i].clone());
            }
        }

        None
    }
}
