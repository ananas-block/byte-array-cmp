use solana_program::pubkey::Pubkey;
use light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64;
use zerocopy::{KnownLayout, Immutable, FromBytes, IntoBytes};
use light_zero_copy::ZeroCopyTraits;

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
    pub fn new(capacity: u64, backing_store: &'a mut [u8]) -> Result<Self, light_zero_copy::errors::ZeroCopyError> {
        Ok(Self {
            entries: ZeroCopyCyclicVecU64::<T>::new(capacity, backing_store)?,
        })
    }
    
    pub fn push(&mut self, entry: T) {
        self.entries.push(entry);
    }
    
    // Search backwards from latest_index for up to num_iters
    // None = search all
    pub fn find_latest(&self, key: T::Key, num_iters: Option<usize>) -> Option<T::Value> {
        let max_iters = num_iters.unwrap_or(self.entries.len()).min(self.entries.len());
        
        if max_iters == 0 || self.entries.is_empty() {
            return None;
        }
        
        // Start from the last inserted element and work backwards
        let mut current_index = self.entries.last_index();
        let mut iterations = 0;
        
        while iterations < max_iters {
            if let Some(entry) = self.entries.get(current_index) {
                if entry.key() == key {
                    return Some(entry.value());
                }
            }
            
            iterations += 1;
            
            if iterations < max_iters {
                // Move to previous index, handling wrap-around
                if current_index == 0 {
                    if self.entries.len() == self.entries.capacity() {
                        // We've wrapped around, go to the end
                        current_index = self.entries.capacity() - 1;
                    } else {
                        // Haven't filled capacity yet, we're done
                        break;
                    }
                } else {
                    current_index -= 1;
                }
            }
        }
        
        None
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
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
    pub fn new(mint: [u8; 32], value: u64) -> Self {
        Self { value, mint }
    }
    
    pub fn new_from_pubkey(mint: Pubkey, value: u64) -> Self {
        Self { value, mint: mint.to_bytes() }
    }
}

impl KeyValue for Entry {
    type Value = u64;
    type Key = [u8; 32];
    
    fn key(&self) -> [u8; 32] {
        self.mint
    }
    
    fn value(&self) -> Self::Value {
        self.value
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
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();
        
        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);
        let mint3 = create_test_pubkey(3);
        
        // Add entries
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint2, 200));
        changelog.push(Entry::new(mint3, 300));
        
        // Test finding latest values
        assert_eq!(changelog.find_latest(mint1, None), Some(100));
        assert_eq!(changelog.find_latest(mint2, None), Some(200));
        assert_eq!(changelog.find_latest(mint3, None), Some(300));
        
        // Test non-existent key
        let mint_not_found = create_test_pubkey(99);
        assert_eq!(changelog.find_latest(mint_not_found, None), None);
    }
    
    #[test]
    fn test_generic_changelog_overwrites() {
        let capacity = 3u64;
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();
        
        let mint1 = create_test_pubkey(1);
        let mint2 = create_test_pubkey(2);
        
        // Add initial entries
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint2, 200));
        changelog.push(Entry::new(mint1, 150)); // Update mint1
        
        // Should find the latest value for mint1
        assert_eq!(changelog.find_latest(mint1, None), Some(150));
        assert_eq!(changelog.find_latest(mint2, None), Some(200));
    }
    
    #[test]
    fn test_generic_changelog_cyclic_behavior() {
        let capacity = 3u64;
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
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
        assert_eq!(changelog.find_latest(mint1, None), None);
        assert_eq!(changelog.find_latest(mint2, None), Some(200));
        assert_eq!(changelog.find_latest(mint3, None), Some(300));
        assert_eq!(changelog.find_latest(mint4, None), Some(400));
    }
    
    #[test]
    fn test_generic_changelog_limited_search() {
        let capacity = 10u64;
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
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
        assert_eq!(changelog.find_latest(mint1, None), Some(100)); // 10 * 10
        assert_eq!(changelog.find_latest(mint2, None), Some(90));  // 9 * 10
        
        // Find latest with limit of 3 iterations
        // Last 3 entries are: mint2(90), mint1(100), mint2(90) (working backwards)
        assert_eq!(changelog.find_latest(mint1, Some(3)), Some(100));
        assert_eq!(changelog.find_latest(mint2, Some(3)), Some(90));
        
        // Find latest with limit of 1 (only check the very last entry)
        assert_eq!(changelog.find_latest(mint1, Some(1)), Some(100)); // Last entry is mint1
        assert_eq!(changelog.find_latest(mint2, Some(1)), None);      // Last entry is not mint2
    }
    
    #[test]
    fn test_edge_cases() {
        let capacity = 5u64;
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        
        // Test empty changelog
        let changelog: GenericChangelog<'_, Entry> = GenericChangelog::new(capacity, &mut backing_store).unwrap();
        let mint1 = create_test_pubkey(1);
        assert_eq!(changelog.find_latest(mint1, None), None);
        
        // Test single entry
        let mut backing_store2 = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store2).unwrap();
        changelog.push(Entry::new(mint1, 42));
        assert_eq!(changelog.find_latest(mint1, None), Some(42));
        assert_eq!(changelog.find_latest(mint1, Some(1)), Some(42));
        assert_eq!(changelog.find_latest(mint1, Some(0)), None);
    }
    
    #[test]
    fn test_reverse_search_order() {
        let capacity = 5u64;
        let mut backing_store = vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
        let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();
        
        let mint1 = create_test_pubkey(1);
        
        // Add multiple entries with same key but different values
        changelog.push(Entry::new(mint1, 100));
        changelog.push(Entry::new(mint1, 200));
        changelog.push(Entry::new(mint1, 300));
        
        // Should find the most recent value (300)
        assert_eq!(changelog.find_latest(mint1, None), Some(300));
        
        // Test that it really is searching in reverse order
        assert_eq!(changelog.find_latest(mint1, Some(1)), Some(300)); // Only check last
        assert_eq!(changelog.find_latest(mint1, Some(2)), Some(300)); // Check last 2, still finds 300
    }
}