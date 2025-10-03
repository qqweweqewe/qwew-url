use std::{collections::HashSet, sync::Arc, thread};

use crate::generator::*;

#[test]
fn test_basic_functionality() {
    let generator = UriGenerator::new("abc", 5);
    let hash1 = generator.next();
    let hash2 = generator.next();
    
    assert_eq!(hash1.len(), 5);
    assert_eq!(hash2.len(), 5);
    assert_ne!(hash1, hash2);
    
    // verify only valid characters are used
    for c in hash1.chars() {
        assert!(matches!(c, 'a' | 'b' | 'c'));
    }
}

#[test]
fn test_uniqueness() {
    let generator = UriGenerator::new("QqWwEe", 16);
    let mut hashes = std::collections::HashSet::new();
    
    for _ in 0..1_000_000 {
        let hash = generator.next();
        assert!(!hashes.contains(&hash), "Duplicate hash found: {}", hash);
        hashes.insert(hash);
    }
}

#[test]
fn test_different_lengths() {
    let alphabets = vec!["abc", "0123456789", "abcdefghijklmnopqrstuvwxyz"];
    
    for alphabet in alphabets {
        for length in 1..=10 {
            let generator = UriGenerator::new(alphabet, length);
            let hash = generator.next();
            assert_eq!(hash.len(), length);
        }
    }
}


#[test]
fn test_thread_safety() {
    let generator = Arc::new(UriGenerator::new("QqWwEe",16));
    let mut handles = vec![];
    let num_threads = 10;
    let iterations_per_thread = 100;

    for _ in 0..num_threads {
        let gen_clone = Arc::clone(&generator);
        let handle = thread::spawn(move || {
            let mut results = vec![];
            for _ in 0..iterations_per_thread {
                results.push(gen_clone.next());
            }
            results
        });
        handles.push(handle);
    }

    let mut all_results = vec![];
    for handle in handles {
        all_results.extend(handle.join().unwrap());
    }

    let unique_results: HashSet<String> = all_results.into_iter().collect();
    assert_eq!(
        unique_results.len(),
        num_threads * iterations_per_thread,
        "All generated URIs should be unique"
    );
}
