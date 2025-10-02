use std::hash::{Hash, Hasher};

pub struct HashGenerator {
    alphabet: Vec<char>,
    alphabet_len: usize,
    length: usize,
    counter: u64,
}

impl HashGenerator {
    pub fn new(alphabet: &str, length: usize) -> Self {
        let alphabet_chars: Vec<char> = alphabet.chars().collect();

        assert!(alphabet.len() != 0, "Alphabet must not be empty");
        assert!(length > 0, "Hash length must be at least 1");

        Self {
            alphabet: alphabet_chars,
            alphabet_len: alphabet.len(),
            length,
            counter: 0,
        }
    }

    pub fn hash<T: Hash>(&mut self, input: T) -> String {
        let mut result = String::with_capacity(self.length);
        
        // combine input hash with counter for uniqueness
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        input.hash(&mut hasher);
        self.counter.hash(&mut hasher);

        let mut hash_value = hasher.finish();

        // generate string
        for _ in 0..self.length {
            let index = (hash_value % self.alphabet_len as u64) as usize;
            result.push(self.alphabet[index]);
            hash_value = hash_value / self.alphabet_len as u64;
            
            // if run out of bits, re-hash with mixing
            if hash_value == 0 {
                let mut new_hasher = std::collections::hash_map::DefaultHasher::new();
                result.hash(&mut new_hasher);
                self.counter.hash(&mut new_hasher);
                hash_value = new_hasher.finish();
            }
        }

        self.counter += 1;

        result
    }
}