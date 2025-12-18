struct MyHashMap {
    map: std::collections::HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            map: std::collections::HashMap::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key, value);
    }

    fn get(&self, key: i32) -> i32 {
        *self.map.get(&key).unwrap_or(&-1)
    }

    fn remove(&mut self, key: i32) {
        self.map.remove(&key);
    }
}
