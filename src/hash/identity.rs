use core::hash::{BuildHasherDefault, Hasher};

use byteorder::{ByteOrder, NativeEndian};

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone, Copy)]
pub struct IdentityHasher {
    hash: u64,
}
impl Hasher for IdentityHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() == 8 {
            self.hash = self.hash.wrapping_add(NativeEndian::read_u64(bytes));
        } else {
            unreachable!()
        }
    }

    #[inline(always)]
    fn write_u8(&mut self, n: u8) {
        self.hash = u64::from(n)
    }

    #[inline(always)]
    fn write_u16(&mut self, n: u16) {
        self.hash = u64::from(n)
    }

    #[inline(always)]
    fn write_u32(&mut self, n: u32) {
        self.hash = u64::from(n)
    }

    #[inline(always)]
    fn write_u64(&mut self, n: u64) {
        self.hash = n
    }

    #[inline(always)]
    fn write_usize(&mut self, n: usize) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn write_i8(&mut self, n: i8) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn write_i16(&mut self, n: i16) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn write_i32(&mut self, n: i32) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn write_i64(&mut self, n: i64) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn write_isize(&mut self, n: isize) {
        self.hash = n as u64
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }
}

pub type BuildIdentityHasher = BuildHasherDefault<IdentityHasher>;

#[cfg(feature = "std")]
pub type IdentityHashMap<K, V> = HashMap<K, V, BuildIdentityHasher>;
#[cfg(feature = "std")]
pub type IdentityHashSet<V> = HashSet<V, BuildIdentityHasher>;
