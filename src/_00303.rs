struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> NumArray {
        let n = nums.len();
        let mut sums = vec![0; n + 1];

        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i];
        }

        Self { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize + 1] - self.sums[left as usize]
    }
}
