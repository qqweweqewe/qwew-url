pub struct UriGenerator {
    alphabet: Vec<char>,
    alphabet_len: usize,
    length: usize,
    counter: u64,
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
            counter: 0,
        }
    }
    
    // aight f that just counting in base-*alphabet length* is reliable enough. yeah boring, so what?
    pub fn next(&mut self) -> String {
        let mut out: Vec<char> = Vec::with_capacity(self.length);
        let mut temp_count = self.counter as usize;

        for _ in 0..self.length {
            out.push(self.alphabet[temp_count % self.alphabet_len]);
            temp_count /= self.alphabet_len;
        }

        self.counter += 1;
        out.iter().collect()
    }
}
