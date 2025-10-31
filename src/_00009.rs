use crate::Solution;

impl Solution {
    // 将 x 改成 mut x 无影响，i32 类型是 copy 类型，方法内部拿到的是副本，这里实际上是创建了一个副本
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverted_number = 0_i32;
        while x > reverted_number {
            reverted_number = reverted_number * 10 + x % 10;
            x /= 10
        }

        x == reverted_number || x == reverted_number / 10
    }
}
