pub struct State {
    pub angles: Vec<f32>,
}

impl State {
    pub fn new(num_arms: usize) -> Self {
        State {
            angles: vec![0.0; num_arms],
        }
    }
}