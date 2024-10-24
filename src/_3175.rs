use crate::Solution;

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let mut i = 0;
        let mut last_i = 0;

        let len = skills.len();
        while i < len {
            let mut j = i + 1;
            while j < len && skills[j] < skills[i] && cnt < k {
                j += 1;
                cnt += 1;
            }

            if cnt == k {
                return i as i32;
            }

            cnt = 1;
            last_i = i;
            i = j;
        }


        last_i as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_find_winning_player() {
        assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }
}