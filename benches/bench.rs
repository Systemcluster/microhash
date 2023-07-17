#![allow(soft_unstable)]
#![feature(test)]

extern crate test;
use test::Bencher;

use std::hash::{BuildHasher, Hasher};
use std::hint::black_box;

const ITERATIONS: usize = 100000;

const TEST_DATA_STRING: &str = "He has the most who is most content with the least";
#[bench]
fn bench_string_ahash(b: &mut Bencher) {
    use ahash::RandomState;

    let hashable = black_box(TEST_DATA_STRING.as_bytes());
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(RandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_string_uahash(b: &mut Bencher) {
    use microhash::AHasherRandomState;

    let hashable = black_box(TEST_DATA_STRING.as_bytes());
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_string_xxhash(b: &mut Bencher) {
    use xxhash_rust::xxh3::Xxh3;

    let hashable = black_box(TEST_DATA_STRING.as_bytes());
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(Xxh3::new());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_string_cxhash(b: &mut Bencher) {
    use xxhash_rust::const_xxh3::xxh3_64;

    let hashable = black_box(TEST_DATA_STRING.as_bytes());
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let _ = black_box(xxh3_64(hashable));
        }
    });
}

const TEST_DATA_BIG: &[u8] = include_bytes!("../Cargo.toml");
#[bench]
fn bench_big_ahash(b: &mut Bencher) {
    use ahash::RandomState;

    let hashable = black_box(TEST_DATA_BIG);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(RandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_big_uahash(b: &mut Bencher) {
    use microhash::AHasherRandomState;

    let hashable = black_box(TEST_DATA_BIG);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_big_xxhash(b: &mut Bencher) {
    use xxhash_rust::xxh3::Xxh3;

    let hashable = black_box(TEST_DATA_BIG);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(Xxh3::new());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_big_cxhash(b: &mut Bencher) {
    use xxhash_rust::const_xxh3::xxh3_64;

    let hashable = black_box(TEST_DATA_BIG);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let _ = black_box(xxh3_64(hashable));
        }
    });
}

const TEST_DATA_ARRAY: &[u8] = &[1, 2, 3, 4, 5];
#[bench]
fn bench_array_ahash(b: &mut Bencher) {
    use ahash::RandomState;

    let hashable = black_box(TEST_DATA_ARRAY);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(RandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_array_uahash(b: &mut Bencher) {
    use microhash::AHasherRandomState;

    let hashable = black_box(TEST_DATA_ARRAY);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(AHasherRandomState::with_seeds(1, 2, 3, 4).build_hasher());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_array_xxhash(b: &mut Bencher) {
    use xxhash_rust::xxh3::Xxh3;

    let hashable = black_box(TEST_DATA_ARRAY);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let mut ahasher = black_box(Xxh3::new());
            let _ = black_box({
                ahasher.write(hashable);
                ahasher.finish()
            });
        }
    });
}
#[bench]
fn bench_array_cxhash(b: &mut Bencher) {
    use xxhash_rust::const_xxh3::xxh3_64;

    let hashable = black_box(TEST_DATA_ARRAY);
    b.iter(|| {
        for _ in 0..ITERATIONS {
            let _ = black_box(xxh3_64(hashable));
        }
    });
}
