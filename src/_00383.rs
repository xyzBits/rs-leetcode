use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut cnt = vec![0; 26];
        for ch in magazine.chars() {
            cnt[ch as usize - 'a' as usize] += 1;
        }

        for ch in ransom_note.chars() {
            cnt[ch as usize - 'a' as usize] -= 1;
            if cnt[ch as usize - 'a' as usize] < 0 {
                return false;
            }
        }

        true
    }
}
