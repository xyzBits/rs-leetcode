#[test]
fn test_001() {
    let s1 = String::from("Hello");
    let s2 = s1;
}

fn calculate_length(s: &String) -> usize {
    // s 只是借用 String 的不可变引用
    s.len()
} // 离开作用域，但它只是引用，所以内存不会被释放

#[test]
fn test_002() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("{}", len);
}

// 变量 s 本身不可变，也就是他的指向不能变，但是他指向的数据可以变
fn change(s: &mut String) {
    s.push_str(" World");
}

// 变量 s 本身指向的数据也可以变
fn change_v2(mut s: &mut String) {
    s.push_str(" World");
}

#[test]
fn test_003() {
    let mut s1 = String::from("Hello");
    change(&mut s1);
    println!("{}", s1);
}

fn process_slices(s: &str) {
    println!("Processed slice length: {}", s.len());
}

#[test]
fn test_004() {
    let owned_data = String::from("Rust is Magic");

    process_slices(&owned_data);

    process_slices("Literal Slice");

    let x = 5;
    let y = &x;

    println!("{}", *y);
    let z = *y + 1;

    let s = String::from("Hello");
    let s_ref = &s;

    let len = s_ref.len();

    let boxed = Box::new(String::from("World"));
    let len = boxed.len();
    let string = *boxed;
}
