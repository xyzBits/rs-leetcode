use crate::Solution;

impl Solution {
    pub fn remove_element_(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;

        for right in 0..nums.len() {
            if nums[right] != val {
                nums[left] = nums[right];
                left += 1;
            }
        }

        left as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            if nums[left] == val {
                nums[left] = nums[right - 1];
                right -= 1;
            } else {
                left += 1;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);

        let time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("error");

        println!("{:?}", time.as_millis());
    }
}