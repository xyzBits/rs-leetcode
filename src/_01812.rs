use crate::Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let coordinates = coordinates.as_bytes();
        coordinates[0] % 2 != coordinates[1] % 2
    }
}
