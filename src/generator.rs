use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct ThreadSafeUniqueStringGenerator {
    charset: Vec<char>,
    counter: AtomicU64,
    length: usize,
}

impl ThreadSafeUniqueStringGenerator {
    pub fn new(charset: &str, length: usize) -> Self {
        Self {
            charset: charset.chars().collect(),
            counter: AtomicU64::new(0),
            length,
        }
    }
    
    pub fn generate(&self) -> String {
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let mut rng = StdRng::seed_from_u64(counter);
        
        (0..self.length)
            .map(|_| self.charset[rng.random_range(0..self.charset.len())])
            .collect()
    }
    
    pub fn get_count(&self) -> u64 {
        self.counter.load(Ordering::SeqCst)
    }
}