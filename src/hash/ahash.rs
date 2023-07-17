use core::hash::{BuildHasher, BuildHasherDefault, Hasher};
use core::mem::transmute;

use const_random::const_random;

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

use crate::util::*;

const INIT: [u8; 32] = unsafe {
    transmute([
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
    ])
};

const MULTIPLE: u64 = 6364136223846793005;
const PI1: [u64; 4] = [
    0x243f_6a88_85a3_08d3,
    0x1319_8a2e_0370_7344,
    0xa409_3822_299f_31d0,
    0x082e_fa98_ec4e_6c89,
];
const PI2: [u64; 4] = [
    0x4528_21e6_38d0_1377,
    0xbe54_66cf_34e9_0c6c,
    0xc0ac_29b7_c97c_50dd,
    0x3f84_d5b5_b547_0917,
];
const ROT: u32 = 23;

#[derive(Debug, Clone, Copy)]
pub struct AHasher {
    buffer:     u64,
    pad:        u64,
    extra_keys: [u64; 2],
}

impl Default for AHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl AHasher {
    #[inline]
    pub const fn new() -> Self {
        Self::from_random_state(&AHasherRandomState::new())
    }

    #[inline]
    pub const fn from_keys(key1: u128, key2: u128) -> Self {
        let pi0: [u128; 2] = unsafe { transmute(PI1) };
        let key1: [u64; 2] = unsafe { transmute(key1 ^ pi0[0]) };
        let key2: [u64; 2] = unsafe { transmute(key2 ^ pi0[1]) };
        Self {
            buffer:     key1[0],
            pad:        key1[1],
            extra_keys: key2,
        }
    }

    #[inline]
    pub const fn from_random_state(state: &AHasherRandomState) -> Self {
        Self {
            buffer:     state.k1,
            pad:        state.k0,
            extra_keys: [state.k2, state.k3],
        }
    }

    #[inline]
    pub const fn hash(data: &[u8]) -> u64 {
        let mut hasher = Self::new();
        hasher.write(data);
        hasher.finish()
    }

    #[inline]
    pub const fn hash_with_keys(data: &[u8], key1: u128, key2: u128) -> u64 {
        let mut hasher = Self::from_keys(key1, key2);
        hasher.write(data);
        hasher.finish()
    }

    #[inline]
    pub const fn write(&mut self, data: &[u8]) {
        let length = data.len();
        self.buffer = self.buffer.wrapping_add(length as u64).wrapping_mul(MULTIPLE);
        let data = data as *const _ as *const u8;
        if length > 16 {
            self.large_update(read_last_u128(unsafe { data.add(length) }));
            let mut index = 0;
            while length > (index + 16) {
                self.large_update(read_u128(unsafe { data.add(index) }));
                index += 16;
            }
        } else if length > 8 {
            self.large_update(unsafe { transmute([read_u64(data), read_last_u64(data.add(length))]) });
        } else {
            let value = read_small(data, unsafe { data.add(length) }, length);
            self.large_update(unsafe { transmute(value) });
        }
    }

    #[inline(always)]
    const fn update(&mut self, new_data: u64) {
        self.buffer = folded_multiply(new_data ^ self.buffer, MULTIPLE);
    }

    #[inline(always)]
    const fn large_update(&mut self, new_data: u128) {
        let block: &[u64; 2] = unsafe { transmute(&new_data) };
        let combined = folded_multiply(block[0] ^ self.extra_keys[0], block[1] ^ self.extra_keys[1]);
        self.buffer = (self.buffer.wrapping_add(self.pad) ^ combined).rotate_left(ROT);
    }

    #[inline(always)]
    pub const fn finish(&self) -> u64 {
        folded_multiply(self.buffer, self.pad).rotate_left((self.buffer & 63) as u32)
    }
}

impl Hasher for AHasher {
    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.update(i as _);
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.update(i as _);
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.update(i as _);
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.update(i as _);
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        self.large_update(i);
    }

    #[inline(always)]
    #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "32",
        target_pointer_width = "16"
    ))]
    fn write_usize(&mut self, i: usize) {
        self.update(i as _);
    }

    #[inline(always)]
    #[cfg(target_pointer_width = "128")]
    fn write_usize(&mut self, i: usize) {
        self.large_update(i as _);
    }

    #[inline(always)]
    fn write(&mut self, input: &[u8]) {
        (self as &mut AHasher).write(input);
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        (self as &AHasher).finish()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct AHasherRandomState {
    k0: u64,
    k1: u64,
    k2: u64,
    k3: u64,
}

impl AHasherRandomState {
    #[inline]
    pub const fn new() -> Self {
        unsafe {
            #[allow(clippy::zero_prefixed_literal)]
            Self {
                k0: *(&INIT[00] as *const _) as _,
                k1: *(&INIT[08] as *const _) as _,
                k2: *(&INIT[16] as *const _) as _,
                k3: *(&INIT[24] as *const _) as _,
            }
        }
    }

    #[inline]
    pub const fn with_seeds(k0: u64, k1: u64, k2: u64, k3: u64) -> Self {
        Self {
            k0: k0 ^ PI2[0],
            k1: k1 ^ PI2[1],
            k2: k2 ^ PI2[2],
            k3: k3 ^ PI2[3],
        }
    }
}

impl Default for AHasherRandomState {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for AHasherRandomState {
    type Hasher = AHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::from_random_state(self)
    }
}

pub type BuildAHasher = BuildHasherDefault<AHasher>;

#[cfg(feature = "std")]
pub type AHashMap<K, V> = HashMap<K, V, BuildAHasher>;
#[cfg(feature = "std")]
pub type AHashSet<V> = HashSet<V, BuildAHasher>;
