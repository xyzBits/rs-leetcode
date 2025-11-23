use crate::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let n = s.len();
        let mut arr = s.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            while i < n && !Self::is_vowel(arr[i]) {
                i += 1;
            }

            while j > 0 && !Self::is_vowel(arr[j]) {
                j -= 1;
            }

            if i < j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        arr.iter().collect()
    }

    fn is_vowel(c: char) -> bool {
        "aeiouAEIOU".contains(c)
    }
}
