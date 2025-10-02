use crate::generator::*;

#[test]
fn test_basic_functionality() {
    let mut generator = HashGenerator::new("abc", 5);
    let hash1 = generator.hash("test1");
    let hash2 = generator.hash("test2");
    
    assert_eq!(hash1.len(), 5);
    assert_eq!(hash2.len(), 5);
    assert_ne!(hash1, hash2);
    
    // Verify only valid characters are used
    for c in hash1.chars() {
        assert!(matches!(c, 'a' | 'b' | 'c'));
    }
}

#[test]
fn test_uniqueness() {
    let mut generator = HashGenerator::new("QqWwEe", 32);
    let mut hashes = std::collections::HashSet::new();
    
    for i in 0..100000 {
        let hash = generator.hash(i);
        assert!(!hashes.contains(&hash), "Duplicate hash found: {}", hash);
        hashes.insert(hash);
    }
}

#[test]
fn test_different_lengths() {
    let alphabets = vec!["abc", "0123456789", "abcdefghijklmnopqrstuvwxyz"];
    
    for alphabet in alphabets {
        for length in 1..=10 {
            let mut generator = HashGenerator::new(alphabet, length);
            let hash = generator.hash("test");
            assert_eq!(hash.len(), length);
        }
    }
}