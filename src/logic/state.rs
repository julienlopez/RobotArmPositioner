#[derive(Clone)]
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

impl std::ops::Add for State {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            angles: self
                .angles
                .iter()
                .zip(other.angles.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl std::ops::AddAssign for State {
    fn add_assign(&mut self, other: Self) {
        for (a, b) in self.angles.iter_mut().zip(other.angles.iter()) {
            *a += b;
        }
    }
}

impl std::ops::Sub for State {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            angles: self
                .angles
                .iter()
                .zip(other.angles.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl std::ops::Mul<f32> for State {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            angles: self.angles.iter().map(|a| a * scalar).collect(),
        }
    }
}

impl std::ops::Mul<State> for f32 {
    type Output = State;

    fn mul(self, state: State) -> State {
        State {
            angles: state.angles.iter().map(|a| a * self).collect(),
        }
    }
}
