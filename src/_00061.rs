use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        for i in (0..len).rev() {
            digits[i] = (digits[i] + 1) % 10;
            if digits[i] != 0 {
                return digits;
            }
        }

        digits = vec![0; len + 1];
        digits[0] = 1;
        digits
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

struct Point {
    x: i32,
    y: i32,
}


#[test]
fn test_001() {
    // 模式 就是 形状
    let msg = Message::Move {x: 1, y: 2};

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        },
        Message::Write(s) => {
            println!("Write to string: {}", s);
        }
    }


    let score = 85;

    match score {
        0 => println!("zero"),
        1..=60 => println!("bad score: {}", score),
        61..=89 => println!("not bad score: {}", score),
        _ => println!("some good score"),
    };


    let config_value = Some(30);
    if let Some(x) = config_value {
        println!("found config value: {}", x);
    }// 忽略 None 值，代码不会进入 if


    let coords = (10, -5);
    // 解构元组
    let (x, y) = coords;
    println!("x: {}, y: {}", x, y);


    let p = Point {x: 200, y: 300};
    let Point {y, ..} = p;
    println!("only need y = {}", y);
}