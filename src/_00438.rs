use crate::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_len = s.len();
        let p_len = p.len();

        if s_len < p_len {
            return vec![];
        }

        let mut ans = vec![];

        let mut s_count = vec![0; 26];
        let mut p_count = vec![0; 26];

        for i in 0..p_len {
            s_count[s.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
            p_count[p.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        }

        if s_count == p_count {
            ans.push(0);
        }

        for i in 0..s_len - p_len {
            s_count[s.chars().nth(i).unwrap() as usize - 'a' as usize] -= 1;
            s_count[s.chars().nth(i + p_len).unwrap() as usize - 'a' as usize] += 1;

            if s_count == p_count {
                ans.push(i as i32 + 1);
            }
        }


        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc")),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams(String::from("abab"), String::from("ab")),
            vec![0, 1, 2]
        );
    }
}
