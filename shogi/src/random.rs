use rand::{Rng, thread_rng};

pub struct Random {
    pub rng: rand::prelude::ThreadRng,
    pub min: u16,
    pub max: u16,
}

impl Random {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn generate_multi(&mut self, length: u16) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::new();
        for _i in 0..length {
            let r = self.rng.gen_range(self.min..=self.max);
            result.push(r);
        }
        return result
    }
}