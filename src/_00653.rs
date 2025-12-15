use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

impl Solution {
    pub fn find_target_(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        // queue.push_back(root);

        queue.push_back(root.unwrap());

        while let Some(node_rc) = queue.pop_front() {
            // let node_rc = queue.pop_front().unwrap().unwrap();
            let node = node_rc.borrow();

            let complement = k - node.val;
            if set.contains(&complement) {
                return true;
            }

            set.insert(node.val);
            if let Some(left_rc) = &node.left {
                queue.push_back(left_rc.clone());
            }
            if let Some(right_rc) = &node.right {
                queue.push_back(right_rc.clone());
            }
        }

        false
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node_rc = queue.pop_front().unwrap().unwrap();
            let node = node_rc.as_ref().borrow();

            if set.contains(&(k - node.val)) {
                return true;
            }

            set.insert(node.val);
            if node.left.is_some() {
                queue.push_back(node.left.clone());
            }
            if node.right.is_some() {
                queue.push_back(node.right.clone());
            }
        }

        false
    }
}

#[test]
fn test_001() {
    let data = vec![1, 2, 3];

    // 编译器会检查  Vec<i32> 是否 T: Send
    std::thread::spawn(move || {
        println!("{:?}", data);
    });
    // Vec<i32> 是 Send 的，所以编译通过
}

#[test]
fn test_002() {
    let value = Rc::new(10);

    // std::thread::spawn(move || {
    //     println!("{:?}", value);
    // }).join().unwrap();

    let safe_value = Arc::new(10);
    let safe_value_clone = safe_value.clone(); // 增加引用计数

    std::thread::spawn(move || {
        println!("{:?}", safe_value_clone);
    })
    .join()
    .unwrap();

    println!("{}", safe_value);
}

#[test]
fn test_003() {
    let s1: &str = "banana";

    let s2: &str = &String::from("banana");

    let arr = [1, 2, 3, 4, 5];

    let s3: &[i32] = &arr[1..3];

    let mut x = "Hello Java";

    // 把 x pin 起来
    let mut pinned_x: Pin<&mut &str> = Pin::new(&mut x);

    *pinned_x = "Hello Rust";

    println!("{}", x);
}

struct MySelfRefStruct {
    data: String,
    // 这个标记让编译器知道，这个结构体移动起来不安全
    _marker: std::marker::PhantomPinned,
}

#[test]
fn test_004() {
    let mut my_struct = MySelfRefStruct {
        data: String::from("Secret"),
        _marker: std::marker::PhantomPinned,
    };

    let mut pinned: Pin<&mut MySelfRefStruct> = unsafe { Pin::new_unchecked(&mut my_struct) };

    // let mutable_ref = pinned.get_mut();
}

async fn my_async_function() {
    let a = 10;
    let ref_to_a = &a; // 自引用，结构体晨的字段引用了另一个字段
    some_io_operation().await; // 挂起 yield

    println!("{}", ref_to_a); // 恢复时使用引用
}

async fn some_io_operation() {}
//  从编译器的视角，上面的代码会被编译成类似下面的结构体
struct MyAsyncFunctionFuture {
    state: i32,
    a: i32,
    ref_to_a: *const i32, // 指向自己的字段 a
}
// 1. 当调用 my_async_function() 时，创建了这个结构体
// 2. 当遇到 .await 时，函数暂停，这个结构体被保存在内存中
// 3. 如果不使用 Pin，假设用户把这个Future移动到另一个变量里，内存地址变了
// 4. ref_to_a 还指向旧的地址
// 5. 当Future 恢复运行时，wake up，去读ref_to_a 直接崩溃或读取脏数据
// 所以 Rust 规则 ，Future::poll 方法必须接收 Pin<&mut Self>
// 这意味着，如果你想执行一个 Future，必须把它Pin住，承诺以后再也不移动它

// java 对象在堆上由 GC 随意移动，引用自动更新，你无感
// rust 对象默认可以被移动，但移动后地址失效，引用不自动更新
// rust 如果对象内部保存了指向自己的指针，一查移动，指针就废了
// Pin 这个对象现在的地址是 X ，我承诺永远不再把它移动到别的地址，直到它被销毁
// Unpin 没有自引用，对我来说没有限制，随便移动

#[derive(Debug)]
struct MyFuture {
    x: i32,
    ptr: *const i32,
}

impl MyFuture {
    fn new(val: i32) -> MyFuture {
        MyFuture {
            x: val,
            ptr: std::ptr::null(), // 先置空
        }
    }

    // let ptr point to x
    fn init(&mut self) {
        let self_ref: *const i32 = &self.x;
        self.ptr = self_ref;
    }
}

#[test]
fn test_005() {
    let mut future_a = MyFuture::new(10);

    future_a.init();
    println!("1. future_a 的地址: {:p}", &future_a);
    println!("   future_a.x 的地址: {:p}", &future_a.x);
    println!("   future_a.ptr 指向: {:p}", future_a.ptr);

    // 此时一切正常：
    // future_a.ptr == &future_a.x  (指针指向自己)

    // =================================================
    // === 步骤 2: 用户把 Future 移动到了另一个变量 ===
    // =================================================
    // 在 Rust 中，这行简单的赋值代码导致了内存拷贝(Move)
    // future_a 的内容被 bitwise copy 到了 future_b
    let future_b = future_a;

    println!("\n--- 发生移动 (Move) ---\n");

    println!("2. future_b 的地址: {:p}", &future_b);
    println!("   future_b.x 的地址: {:p}", &future_b.x);
    println!("   future_b.ptr 指向: {:p}", future_b.ptr);

    println!("\n结论:");
    if std::ptr::eq(future_b.ptr, &future_b.x) {
        println!("✅ 安全：指针仍然指向新的 x");
    } else {
        println!("❌ 崩溃：指针指向了旧地址 (悬垂指针)!");
        println!("   它指向的是 future_a (已失效) 的位置，而不是 future_b 的位置");
    }
}

type BoxedFuture = Pin<Box<dyn Future<Output = String> + Send>>;
// 不同的 Async 代码块当作同一个类型返回，需要擦除类型
fn get_user_name(id: i32) -> BoxedFuture {
    if id == 1 {
        Box::pin(async { "Admin".to_string() })
    } else {
        Box::pin(async { "User".to_string() })
    }
}
