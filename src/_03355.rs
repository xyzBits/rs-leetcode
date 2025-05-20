use crate::Solution;

impl Solution {
    pub fn is_zero_array(num: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut delta_array = vec![0; num.len() + 1];

        for query in &queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            delta_array[left] += 1;
            delta_array[right + 1] -= 1;
        }

        let mut operation_counts = vec![0; delta_array.len()];
        let mut current_operations = 0;
        for i in 0..delta_array.len() {
            current_operations += delta_array[i];
            operation_counts[i] = current_operations;
        }

        for i in 0..num.len() {
            if operation_counts[i] < num[i] {
                return false;
            }
        }

        true
    }
}
