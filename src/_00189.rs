use crate::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;

        nums.reverse();
        nums[0..k].reverse();
        nums[k..n].reverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let mut solution = Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn test_reverse() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        println!("{:?}", nums);
        nums.reverse();
        println!("{:?}", nums);

        nums.rotate_right(3);
        println!("{:?}", nums);

        let x = 10;
        let r = &x;
        assert_eq!(*r, 10);

        let mut y = 32;
        let m = &mut y;
        *m += 32;
        assert_eq!(*m, 64);
    }
}
