use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();

        let mut dequeue = VecDeque::new();

        for i in 0..k as usize {
            while !dequeue.is_empty() && nums[i] >= nums[*dequeue.back().unwrap() as usize] {
                dequeue.pop_back();
            }
            dequeue.push_back(i as i32);
        }

        let mut ans = vec![0; n - k as usize + 1];
        ans[0] = nums[*dequeue.front().unwrap() as usize];
        for i in k as usize..n {
            while !dequeue.is_empty() && nums[i] >= nums[*dequeue.back().unwrap() as usize] {
                dequeue.pop_back();
            }

            dequeue.push_back(i as i32);
            while *dequeue.front().unwrap() <= i as i32 - k {
                dequeue.pop_front();
            }

            ans[i - k as usize + 1] = nums[*dequeue.front().unwrap() as usize];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }
}
