use crate::generator::*;

#[test]
fn test_basic_functionality() {
    let mut generator = UriGenerator::new("abc", 5);
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
    let mut generator = UriGenerator::new("QqWwEe", 16);
    let mut hashes = std::collections::HashSet::new();
    
    for _ in 0..10_000_000 {
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
            let mut generator = UriGenerator::new(alphabet, length);
            let hash = generator.next();
            assert_eq!(hash.len(), length);
        }
    }
}
