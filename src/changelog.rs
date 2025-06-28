use light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64;
use light_zero_copy::ZeroCopyTraits;
use solana_program::pubkey::Pubkey;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

// Trait for manual comparison
pub trait ManualCompare: Copy {
    fn manual_compare(&self, other: &Self) -> bool;
}

impl ManualCompare for [u8; 32] {
    #[inline(always)]
    fn manual_compare(&self, other: &Self) -> bool {
        for i in 0..32 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

// Optimization 1: Unrolled comparison
#[inline(always)]
pub fn unrolled_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a[0] == b[0]
        && a[1] == b[1]
        && a[2] == b[2]
        && a[3] == b[3]
        && a[4] == b[4]
        && a[5] == b[5]
        && a[6] == b[6]
        && a[7] == b[7]
        && a[8] == b[8]
        && a[9] == b[9]
        && a[10] == b[10]
        && a[11] == b[11]
        && a[12] == b[12]
        && a[13] == b[13]
        && a[14] == b[14]
        && a[15] == b[15]
        && a[16] == b[16]
        && a[17] == b[17]
        && a[18] == b[18]
        && a[19] == b[19]
        && a[20] == b[20]
        && a[21] == b[21]
        && a[22] == b[22]
        && a[23] == b[23]
        && a[24] == b[24]
        && a[25] == b[25]
        && a[26] == b[26]
        && a[27] == b[27]
        && a[28] == b[28]
        && a[29] == b[29]
        && a[30] == b[30]
        && a[31] == b[31]
}

// Optimization 2: SIMD-style u64 chunk comparison
#[inline(always)]
pub fn simd_style_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a_chunks = unsafe { std::slice::from_raw_parts(a.as_ptr() as *const u64, 4) };
    let b_chunks = unsafe { std::slice::from_raw_parts(b.as_ptr() as *const u64, 4) };

    a_chunks[0] == b_chunks[0]
        && a_chunks[1] == b_chunks[1]
        && a_chunks[2] == b_chunks[2]
        && a_chunks[3] == b_chunks[3]
}

// Optimization 3: Branchless comparison using bitwise operations
#[inline(always)]
pub fn branchless_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

// Optimization 4: Fast early-exit with unsafe indexing
#[inline(always)]
pub fn unsafe_fast_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    unsafe {
        let a_ptr = a.as_ptr();
        let b_ptr = b.as_ptr();

        for i in 0..32 {
            if *a_ptr.add(i) != *b_ptr.add(i) {
                return false;
            }
        }
    }
    true
}

// P-Token inspired optimizations

// Optimization 5: sol_memcmp (Solana's optimized memory compare)
#[inline(always)]
pub fn sol_memcmp_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    use solana_program::program_memory::sol_memcmp;
    sol_memcmp(a, b, 32) == 0
}

// Optimization 6: u128 casting for bulk comparison (inspired by p-token patterns)
#[inline(always)]
pub fn u128_cast_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    unsafe {
        let a_u128 = std::slice::from_raw_parts(a.as_ptr() as *const u128, 2);
        let b_u128 = std::slice::from_raw_parts(b.as_ptr() as *const u128, 2);
        a_u128[0] == b_u128[0] && a_u128[1] == b_u128[1]
    }
}

// Optimization 7: Pointer equality fast path (p-token pattern)
#[inline(always)]
pub fn pointer_equality_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    // Fast path: check if same memory location
    if std::ptr::eq(a, b) {
        return true;
    }
    // Fallback to content comparison
    u128_cast_compare(a, b)
}

// Optimization 8: Combined sol_memcmp with early pointer check
#[inline(always)]
pub fn combined_fast_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    // Fast path: pointer equality
    if std::ptr::eq(a, b) {
        return true;
    }
    // Use Solana's optimized memcmp
    use solana_program::program_memory::sol_memcmp;
    sol_memcmp(a, b, 32) == 0
}

// Optimization 9: SIMD-style with iteration (your suggestion)
#[inline(always)]
pub fn simd_iterator_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a_chunks = unsafe { std::slice::from_raw_parts(a.as_ptr() as *const u64, 4) };
    let b_chunks = unsafe { std::slice::from_raw_parts(b.as_ptr() as *const u64, 4) };

    // Iterate over chunks with early exit
    for i in 0..4 {
        if a_chunks[i] != b_chunks[i] {
            return false;
        }
    }
    true
}

// Optimization 10: SIMD-style with zip iterator (more idiomatic)
#[inline(always)]
pub fn simd_zip_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a_chunks = unsafe { std::slice::from_raw_parts(a.as_ptr() as *const u64, 4) };
    let b_chunks = unsafe { std::slice::from_raw_parts(b.as_ptr() as *const u64, 4) };

    // Use iterator with all() for early exit
    a_chunks.iter().zip(b_chunks.iter()).all(|(a, b)| a == b)
}

// Optimization 11: SIMD with slice comparison (let Rust optimize)
#[inline(always)]
pub fn simd_slice_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let a_chunks = unsafe { std::slice::from_raw_parts(a.as_ptr() as *const u64, 4) };
    let b_chunks = unsafe { std::slice::from_raw_parts(b.as_ptr() as *const u64, 4) };

    // Direct slice comparison
    a_chunks == b_chunks
}

pub trait KeyValue {
    type Key: PartialEq;
    type Value: Copy;

    fn key(&self) -> Self::Key;
    fn value(&self) -> Self::Value;
}

/// Size: 8 + 4 + Entry::LEN * entries
pub struct GenericChangelog<'a, T: KeyValue + ZeroCopyTraits> {
    /// Once full index resets and starts at 0 again
    /// existing values are overwritten.
    pub entries: ZeroCopyCyclicVecU64<'a, T>,
}

impl<'a, T: KeyValue + ZeroCopyTraits> GenericChangelog<'a, T> {
    #[inline(always)]
    pub fn new(
        capacity: u64,
        backing_store: &'a mut [u8],
    ) -> Result<Self, light_zero_copy::errors::ZeroCopyError> {
        Ok(Self {
            entries: ZeroCopyCyclicVecU64::<T>::new(capacity, backing_store)?,
        })
    }

    #[inline(always)]
    pub fn from_bytes(
        backing_store: &'a mut [u8],
    ) -> Result<Self, light_zero_copy::errors::ZeroCopyError> {
        Ok(Self {
            entries: ZeroCopyCyclicVecU64::<T>::from_bytes(backing_store)?,
        })
    }

    #[inline(always)]
    pub fn push(&mut self, entry: T) {
        self.entries.push(entry);
    }

    // Search backwards from latest_index for up to num_iters
    // None = search all
    // USE_MANUAL_COMPARISON: true = manual loop, false = rust built-in ==
    #[inline(always)]
    pub fn find_latest<const USE_MANUAL_COMPARISON: bool>(
        &self,
        key: T::Key,
        num_iters: Option<usize>,
    ) -> Option<T::Value>
    where
        T::Key: ManualCompare,
    {
        let iter = if let Some(num_iters) = num_iters {
            self.entries.iter().rev().take(num_iters)
        } else {
            self.entries.iter().rev().take(self.entries.len())
        };
        for entry in iter {
            let keys_match = if USE_MANUAL_COMPARISON {
                // Manual loop comparison with early exit
                entry.key().manual_compare(&key)
            } else {
                // Rust built-in comparison
                entry.key() == key
            };
            if keys_match {
                return Some(entry.value());
            }
        }

        None
    }

    // Optimization variants - Direct comparison methods without trait overhead
    #[inline(always)]
    pub fn find_latest_unrolled(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if unrolled_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_simd(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if simd_style_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_branchless(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if branchless_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_unsafe(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if unsafe_fast_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    // P-Token inspired optimization methods
    #[inline(always)]
    pub fn find_latest_sol_memcmp(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if sol_memcmp_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_u128_cast(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if u128_cast_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_pointer_equality(
        &self,
        key: [u8; 32],
        num_iters: Option<usize>,
    ) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if pointer_equality_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_combined_fast(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if combined_fast_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    // Additional SIMD iteration variants
    #[inline(always)]
    pub fn find_latest_simd_iterator(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if simd_iterator_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    // Detailed CU tracking version for debugging
    #[inline(always)]
    pub fn find_latest_simd_iterator_with_cu_tracking(&self, key: [u8; 32]) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < self.entries.len() {
            let entry = &self.entries[current_index];
            if simd_iterator_compare(&entry.key(), &key) {
                return Some(entry.value());
            }

            iterations += 1;
            if iterations < self.entries.len() {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_simd_zip(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if simd_zip_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn find_latest_simd_slice(&self, key: [u8; 32], num_iters: Option<usize>) -> Option<u64>
    where
        T: KeyValue<Key = [u8; 32], Value = u64>,
    {
        let max_iters = num_iters
            .unwrap_or(self.entries.len())
            .min(self.entries.len());

        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }

        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if simd_slice_compare(&entry.key(), &key) {
                    return Some(entry.value());
                }
            }

            iterations += 1;
            if iterations < max_iters {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }
}

#[derive(Copy, Clone, KnownLayout, Immutable, FromBytes, IntoBytes)]
#[repr(C)]
pub struct Entry {
    pub value: u64,
    pub mint: [u8; 32],
}

impl Entry {
    #[inline(always)]
    pub fn new(mint: [u8; 32], value: u64) -> Self {
        Self { value, mint }
    }

    #[inline(always)]
    pub fn new_from_pubkey(mint: Pubkey, value: u64) -> Self {
        Self {
            value,
            mint: mint.to_bytes(),
        }
    }
}

impl KeyValue for Entry {
    type Value = u64;
    type Key = [u8; 32];

    #[inline(always)]
    fn key(&self) -> [u8; 32] {
        self.mint
    }

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.value
    }
}

// Specific implementation for Entry type with direct field access
impl GenericChangelog<'_, Entry> {
    // Non-generic version that directly accesses Entry struct fields
    #[inline(always)]
    pub fn find_latest_direct_field_access(&self, key: [u8; 32]) -> Option<u64> {
        let mut current_index = self.entries.last_index();
        let mut iterations = 0;

        while iterations < self.entries.len() {
            let entry = &self.entries[current_index];
            // Direct field access instead of trait methods
            if simd_iterator_compare(&entry.mint, &key) {
                return Some(entry.value);
            }

            iterations += 1;
            if iterations < self.entries.len() {
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        current_index = self.entries.capacity() - 1;
                    } else {
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64;

    fn create_test_pubkey(seed: u8) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        bytes
    }

    #[test]
    fn test_generic_changelog_basic() {
        let capacity = 5u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);
        let mint3 = create_test_pubkey(3);

        // Add entries
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint2, 200));
        changelog.push(Entry::new(mint3, 300));

        // Test finding latest values
        assert_eq!(changelog.find_latest::<false>(mint1, None), Some(100));
        assert_eq!(changelog.find_latest::<false>(mint2, None), Some(200));
        assert_eq!(changelog.find_latest::<false>(mint3, None), Some(300));

        // Test non-existent key
        let mint_not_found = create_test_pubkey(99);
        assert_eq!(changelog.find_latest::<false>(mint_not_found, None), None);
    }

    #[test]
    fn test_generic_changelog_overwrites() {
        let capacity = 3u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);

        // Add initial entries
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint2, 200));
        changelog.push(Entry::new(mint1, 150)); // Update mint1

        // Should find the latest value for mint1
        assert_eq!(changelog.find_latest::<false>(mint1, None), Some(150));
        assert_eq!(changelog.find_latest::<false>(mint2, None), Some(200));
    }

    #[test]
    fn test_generic_changelog_cyclic_behavior() {
        let capacity = 3u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);
        let mint3 = create_test_pubkey(3);
        let mint4 = create_test_pubkey(4);

        // Fill the changelog
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint2, 200));
        changelog.push(Entry::new(mint3, 300));

        // Add more entries (should wrap around)
        changelog.push(Entry::new(mint4, 400)); // Overwrites mint1

        // mint1 should no longer be found
        assert_eq!(changelog.find_latest::<false>(mint1, None), None);
        assert_eq!(changelog.find_latest::<false>(mint2, None), Some(200));
        assert_eq!(changelog.find_latest::<false>(mint3, None), Some(300));
        assert_eq!(changelog.find_latest::<false>(mint4, None), Some(400));
    }

    #[test]
    fn test_generic_changelog_limited_search() {
        let capacity = 10u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);

        // Add multiple entries
        for i in 1..=10 {
            if i % 2 == 0 {
                changelog.push(Entry::new(mint1, i * 10));
            } else {
                changelog.push(Entry::new(mint2, i * 10));
            }
        }

        // Find latest with no limit (should find most recent)
        assert_eq!(changelog.find_latest::<false>(mint1, None), Some(100)); // 10 * 10
        assert_eq!(changelog.find_latest::<false>(mint2, None), Some(90)); // 9 * 10

        // Find latest with limit of 3 iterations
        // Last 3 entries are: mint2(90), mint1(100), mint2(90) (working backwards)
        assert_eq!(changelog.find_latest::<false>(mint1, Some(3)), Some(100));
        assert_eq!(changelog.find_latest::<false>(mint2, Some(3)), Some(90));

        // Find latest with limit of 1 (only check the very last entry)
        assert_eq!(changelog.find_latest::<false>(mint1, Some(1)), Some(100)); // Last entry is mint1
        assert_eq!(changelog.find_latest::<false>(mint2, Some(1)), None); // Last entry is not mint2
    }

    #[test]
    fn test_edge_cases() {
        let capacity = 5u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];

        // Test empty changelog
        let changelog: GenericChangelog<'_, Entry> =
            GenericChangelog::new(capacity, &mut backing_store).unwrap();
        let mint1 = create_test_pubkey(1);
        assert_eq!(changelog.find_latest::<false>(mint1, None), None);

        // Test single entry
        let mut backing_store2 =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store2).unwrap();
        changelog.push(Entry::new(mint1, 42));
        assert_eq!(changelog.find_latest::<false>(mint1, None), Some(42));
        assert_eq!(changelog.find_latest::<false>(mint1, Some(1)), Some(42));
        assert_eq!(changelog.find_latest::<false>(mint1, Some(0)), None);
    }

    #[test]
    fn test_reverse_search_order() {
        let capacity = 5u64;
        let mut backing_store =
            vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

        let mint1 = create_test_pubkey(1);

        // Add multiple entries with same key but different values
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint1, 200));
        changelog.push(Entry::new(mint1, 300));

        // Should find the most recent value (300)
        assert_eq!(changelog.find_latest::<false>(mint1, None), Some(300));

        // Test that it really is searching in reverse order
        assert_eq!(changelog.find_latest::<false>(mint1, Some(1)), Some(300)); // Only check last
        assert_eq!(changelog.find_latest::<false>(mint1, Some(2)), Some(300)); // Check last 2, still finds 300
    }
}
