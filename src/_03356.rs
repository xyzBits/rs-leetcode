use crate::Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = queries.len();

        if !Self::check_3356(&nums, &queries, right) {
            return -1;
        }

        while left < right {
            let k = (left + right) / 2;
            if Self::check_3356(&nums, &queries, k) {
                right = k;
            } else {
                left = k + 1;
            }
        }

        left as i32
    }

    fn check_3356(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool {
        let mut delta_array = vec![0; nums.len() + 1];
        for i in 0..k {
            let left = queries[i][0] as usize;
            let right = queries[i][1] as usize;
            let value = queries[i][2];
            delta_array[left] += value;
            delta_array[right + 1] -= value;
        }
        let mut operation_counts = Vec::with_capacity(delta_array.len());
        let mut current_operations = 0;
        for &delta in &delta_array {
            current_operations += delta;
            operation_counts.push(current_operations);
        }
        for i in 0..nums.len() {
            if operation_counts[i] < nums[i] {
                return false;
            }
        }
        true
    }
}
