#[cfg(not(any(target_feature = "aes", target_feature = "crypto")))]
#[test]
fn test_equal_hash_string() {
    use ahash::RandomState as RefRandomState;
    use microhash::AHasherRandomState;
    use std::hash::{BuildHasher, Hasher};

    let hashable = "He has the most who is most content with the least".as_bytes();
    dbg!(hashable.len());
    let a = {
        let mut ahasher = AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };
    let b = {
        let mut ahasher = RefRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };

    assert_eq!(a, b);
}

#[cfg(not(any(target_feature = "aes", target_feature = "crypto")))]
#[test]
fn test_equal_hash_array() {
    use ahash::RandomState as RefRandomState;
    use microhash::AHasherRandomState;
    use std::hash::{BuildHasher, Hasher};

    let hashable = &[1, 2, 3, 4];
    dbg!(hashable.len());
    let a = {
        let mut ahasher = AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };
    let b = {
        let mut ahasher = RefRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };

    assert_eq!(a, b);
}

#[cfg(not(any(target_feature = "aes", target_feature = "crypto")))]
#[test]
fn test_equal_hash_array_small() {
    use ahash::RandomState as RefRandomState;
    use microhash::AHasherRandomState;
    use std::hash::{BuildHasher, Hasher};

    let hashable = &[1, 2, 3];
    dbg!(hashable.len());
    let a = {
        let mut ahasher = AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };
    let b = {
        let mut ahasher = RefRandomState::with_seeds(1, 2, 3, 4).build_hasher();
        dbg!(&ahasher);
        ahasher.write(hashable);
        dbg!(&ahasher);
        ahasher.finish()
    };

    assert_eq!(a, b);
}
