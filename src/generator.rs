use std::sync::atomic::{AtomicU64, Ordering};

pub struct UriGenerator {
    alphabet: Vec<char>,
    alphabet_len: usize,
    length: usize,
    counter: AtomicU64,
}

impl UriGenerator {
    pub fn new(alphabet: &str, length: usize) -> Self {
        let alphabet_chars: Vec<char> = alphabet.chars().collect();

        assert!(!alphabet.is_empty(), "Alphabet must not be empty");
        assert!(length > 0, "Hash length must be at least 1");

        Self {
            alphabet: alphabet_chars,
            alphabet_len: alphabet.len(),
            length,
            counter: AtomicU64::new(0),
        }
    }

    pub fn next(&self) -> String {
        let current_count = self.counter.fetch_add(1, Ordering::Relaxed) as usize;
        let mut out: Vec<char> = Vec::with_capacity(self.length);
        let mut temp_count = current_count;

        for _ in 0..self.length {
            out.push(self.alphabet[temp_count % self.alphabet_len]);
            temp_count /= self.alphabet_len;
        }

        out.iter().collect()
    }
}