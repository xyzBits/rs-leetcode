use crate::Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        let n = chars.len();

        for i in (0..n).step_by((2 * k) as usize) {
            let end = (i + k as usize).min(n);
            chars[i..end].reverse();
        }

        chars.into_iter().collect::<String>()
    }
}

#[test]
fn test_001() {
    let mut v = [1, 2, 3];
    v.reverse();
    assert_eq!(v, [3, 2, 1]);

    v[..2].reverse();
    println!("{:?}", v);

    let mut data = vec![1, 2, 3, 4, 5];
    data[1..3].reverse();
    println!("{:?}", data);

}

#[test]
fn test_002() {
    let word = "goodbye";
    let count = word.chars().count();
    assert_eq!(count, 7);

    let mut chars = word.chars();
    assert_eq!(Some('g'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(None, chars.next());
}


#[test]
fn test_003() {
    let a = [0, 1, 2, 3, 4, 5];
    let mut iter = a.into_iter().step_by(2);
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(None, iter.next());
}