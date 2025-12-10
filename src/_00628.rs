use crate::Solution;

impl Solution {
    pub fn maximum_product_(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        (nums[0] * nums[1] * nums[n - 1]).max(nums[n - 3] * nums[n - 2] * nums[n - 1])
    }

    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let (mut min1, mut min2) = (i32::MAX, i32::MAX);
        let (mut max1, mut max2, mut max3) = (i32::MIN, i32::MIN, i32::MIN);

        for x in nums {
            if x < min1 {
                min2 = min1;
                min1 = x;
            } else if x < min2 {
                min2 = x;
            }

            if x > max1 {
                max3 = max2;
                max2 = max1;
                max1 = x;
            } else if x > max2 {
                max3 = max2;
                max2 = x;
            } else if x > max3 {
                max3 = x;
            }
        }

        (min1 * min2 * max1).max(max1 * max2 * max3)
    }
}
