use rand::{Rng, thread_rng};

pub struct Random {
    pub rng: rand::prelude::ThreadRng,
    pub min: u16,
    pub max: u16,
}

impl Random {
    pub fn init() -> Self {
        Self {
            rng: thread_rng(),
            min: 0,
            max: 9,
        }
    }

    #[allow(dead_code)]
    pub fn new(min: u16, max: u16) -> Self {
        Self {
            rng: thread_rng(),
            min: min,
            max: max,
        }
    }

    #[allow(dead_code)]
    pub fn generate_one(&mut self) -> u16 {
        return self.rng.gen_range(self.min..=self.max)
    }
}