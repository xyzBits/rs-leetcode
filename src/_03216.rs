use crate::Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut char_arr = s.chars().collect::<Vec<char>>();

        for i in 0..char_arr.len() - 1 {
            if char_arr[i] > char_arr[i + 1]
                && (char_arr[i] as u32) % 2 == (char_arr[i + 1] as u32) % 2
            {
                let temp = char_arr[i];
                char_arr[i] = char_arr[i + 1];
                char_arr[i + 1] = temp;
                break;
            }
        }

        char_arr.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_smallest_string() {
        assert_eq!(Solution::get_smallest_string("45320".to_string()), "43520");
        assert_eq!(Solution::get_smallest_string("001".to_string()), "001");
    }

    #[test]
    fn test_iter() {
        let data = vec![1, 3, 43];

        // let iter = data.into_iter();
        // println!("{:?}", data);

        let iter = data.iter();
        println!("{:?}", data);
    }
}
