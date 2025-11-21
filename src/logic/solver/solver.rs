use crate::logic::infra::Infra;
use crate::logic::State;

// various implementations, random, gradient descent, CCD, FABRIK, PSO, ...
pub trait Solver {
    fn solve(infra: &Infra, target: (f32, f32)) -> Option<State>;
}
