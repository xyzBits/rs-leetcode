use crate::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() < 2 {
            return true;
        }

        let mut low = 0;
        let mut high = chars.len() - 1;
        while low < high {
            let c1 = chars[low];
            let c2 = chars[high];

            if c1 == c2 {
                low += 1;
                high -= 1;
            } else {
                let skip_left = Self::valid_palindrome_helper(&chars, low + 1, high);
                let skip_right = Self::valid_palindrome_helper(&chars, low, high - 1);
                return skip_left || skip_right;
            }
        }

        true
    }

    fn valid_palindrome_helper(chars: &[char], mut low: usize, mut high: usize) -> bool {
        while low < high {
            if chars[low] != chars[high] {
                return false;
            }
            low += 1;
            high -= 1;
        }
        true
    }
}
