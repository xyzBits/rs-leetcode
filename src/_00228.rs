use crate::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret = Vec::new();

        let mut i = 0;
        let n = nums.len();
        while i < n {
            let low = i;
            i += 1;
            while i < n && nums[i] == nums[i - 1] + 1 {
                i += 1;
            }

            let high = i - 1;
            let mut temp = nums[low].to_string();
            if low < high {
                temp.push_str("->");
                temp.push_str(nums[high].to_string().as_str());
            }
            ret.push(temp);
        }

        ret
    }
}
