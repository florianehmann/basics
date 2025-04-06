use std::hash::{DefaultHasher, Hash, Hasher};

const DEFAULT_CAPACITY: usize = 16;

pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    /// Creates an empty hash table.
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(DEFAULT_CAPACITY);
        for _ in 0..DEFAULT_CAPACITY {
            buckets.push(Vec::new());
        }

        Self { buckets }
    }

    /// Returns the index of the bucket vector, which is supposed to hold the value for `key`
    /// within the `buckets` vector of all buckets of the hash table.
    fn get_bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    /// Inserts `value` at `key` into the hash table.
    /// If there is already a value at that key, it will be overwritten.
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[index];

        for (ref existing_key, existing_value) in bucket.iter_mut() {
            if *existing_key == key {
                *existing_value = value;
                return;
            }
        }

        bucket.push((key, value));
    }

    /// Returns the `value` associated with `key`, if present.
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_bucket_index(key);
        let bucket = &self.buckets[index];

        for (existing_key, value) in bucket {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }
}

impl<K: Eq + Hash, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let _: HashTable<i32, i32> = HashTable::new();
    }

    #[test]
    fn test_insert_get() {
        let mut ht: HashTable<i32, i32> = HashTable::new();
        ht.insert(5, 42);
        assert_eq!(ht.get(&5), Some(42).as_ref())
    }
}
