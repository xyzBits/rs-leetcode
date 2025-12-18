struct MyHashSet {
    buckets: Vec<Vec<i32>>,
    base: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        let base = 769;
        let buckets = vec![vec![]; base as usize];
        MyHashSet { buckets, base }
    }

    fn hash(&self, key: i32) -> usize {
        (key % self.base) as usize
    }

    fn add(&mut self, key: i32) {
        let h = self.hash(key);
        let bucket = &mut self.buckets[h];
        if !bucket.contains(&key) {
            bucket.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let h = self.hash(key);
        let bucket = &mut self.buckets[h];
        if let Some(ops) = bucket.iter().position(|&x| x == key) {
            bucket.remove(ops);
        }
    }

    fn contains(&self, key: i32) -> bool {
        let h = self.hash(key);
        self.buckets[h].contains(&key)
    }
}
