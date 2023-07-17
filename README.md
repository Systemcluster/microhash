# microhash

Collection of hash implementations with `const` and `no_std` support.

Includes the following hash functions:

- `AHasher` based on [AHash](https://github.com/tkaitchuck/aHash) with changes to support `const` hashing
- `IdentityHasher` for already hashed values

Requires nightly Rust.
