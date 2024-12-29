use rand::prelude::{ThreadRng, SliceRandom};

pub struct PasswordGenerator {
    rng: ThreadRng,
    writable_symbols: Vec<char>,
    length: usize,
    count: usize,
}

impl PasswordGenerator {
    pub fn new(length: usize, count: usize) -> PasswordGenerator {
        PasswordGenerator {
            rng: rand::thread_rng(),
            writable_symbols: ('!'..='~').collect(),
            length,
            count,
        }
    }

    pub fn generate_passwords(&mut self) -> Vec<String> {
        (0..self.count).map(|_| self.generate_password()).collect()
    }

    fn generate_password(&mut self) -> String {
        (0..self.length)
            .map(|_| *self.writable_symbols.choose(&mut self.rng).unwrap())
            .collect()
    }
}
