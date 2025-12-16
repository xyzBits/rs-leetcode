use rand::Rng;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> KthLargest {
        let k_size = k as usize;
        let min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let mut instance = KthLargest {
            k: k_size,
            min_heap,
        };

        for num in nums {
            instance.add(num);
        }

        instance
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));

        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }

        self.min_heap.peek().unwrap().0
    }
}

#[test]
fn test_001() {
    let mut max_heap = BinaryHeap::new();

    max_heap.push(5);
    max_heap.push(1);
    max_heap.push(9);
    max_heap.push(2);

    println!("max element: {:?}", max_heap.peek());

    println!("pop: {:?}", max_heap.pop());
    println!("pop: {:?}", max_heap.pop());
}

#[test]
fn test_002() {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(9));

    println!("min: {:?}", min_heap.peek());
    println!("pop: {:?}", min_heap.pop());
    println!("pop: {:?}", min_heap.pop());
    println!("pop: {:?}", min_heap.pop());
}

// ===============================================
// 1. 横向堆可视化打印函数
// ===============================================

/// 递归地打印堆（数组）的子树结构 (横向)。
/// T: 必须是实现了 Debug trait的类型，以便能被打印。
/// 注意：这里 T 是切片元素的类型，在 main 中，T 实际上是 &i32。
fn print_heap_node_horizontal<T: std::fmt::Debug>(heap_vec: &[T], index: usize, prefix: String) {
    if index >= heap_vec.len() {
        return;
    }

    // 计算左右子节点的索引
    let left_child_index = 2 * index + 1;
    let right_child_index = 2 * index + 2;

    // ----------------------------------------------------
    // 1. 递归打印右子节点 (出现在当前节点的上方)
    // ----------------------------------------------------
    if right_child_index < heap_vec.len() {
        // 构建下一个前缀：如果当前节点不是根节点，且当前节点是左孩子，使用空格；否则使用 '│'
        let new_prefix = format!("{}{}   ", prefix, "│");
        print_heap_node_horizontal(heap_vec, right_child_index, new_prefix);
    }

    // ----------------------------------------------------
    // 2. 打印当前节点
    // ----------------------------------------------------

    // 打印当前节点的缩进、连接符和值
    println!(
        "{}{} {:?}",
        prefix,
        if index == 0 {
            "──" // 根节点
        } else if index % 2 == 0 {
            "┌──" // 左孩子 (在数组中索引为偶数时，是右子树的左孩子，对应父节点的右孩子)
        } else {
            "└──" // 右孩子 (在数组中索引为奇数，对应父节点的左孩子)
        },
        heap_vec[index]
    );

    // ----------------------------------------------------
    // 3. 递归打印左子节点 (出现在当前节点的下方)
    // ----------------------------------------------------
    if left_child_index < heap_vec.len() {
        // 构建下一个前缀：如果当前节点是左孩子，使用空格；否则使用 '│'
        let new_prefix = format!("{}{}   ", prefix, "│");
        print_heap_node_horizontal(heap_vec, left_child_index, new_prefix);
    }
}

// ===============================================
// 2. 主函数：生成、填充与打印
// ===============================================

#[test]
fn test() {
    let num_count = 1000;
    let mut heap = BinaryHeap::new();
    let mut rng = rand::thread_rng();

    // 限制打印的深度，只打印前 4 层 (共 15 个节点)
    let max_levels = 4;
    // 2^4 - 1 = 15
    let max_nodes_to_print = (2u32.pow(max_levels) - 1) as usize;

    println!(
        "开始生成 {} 个随机数 (范围 1 到 10000) 并填充到 BinaryHeap (最大堆)...",
        num_count
    );

    // 填充 1000 个随机数
    for _ in 0..num_count {
        let random_val = rng.gen_range(1..=10000);
        heap.push(random_val);
    }

    // 1. 获取 Vec<&i32> 的表示 (heap.iter() 返回的是元素的引用迭代器)
    let full_vec_representation: Vec<&i32> = heap.iter().collect();

    // 2. 限制要打印的切片范围 (这是 &[&i32] 类型)
    let display_slice = if full_vec_representation.len() > max_nodes_to_print {
        &full_vec_representation[0..max_nodes_to_print]
    } else {
        &full_vec_representation
    };

    println!(
        "\n--- BinaryHeap 横向树状可视化 (仅显示前 {} 个节点) ---",
        display_slice.len()
    );

    // 3. 直接调用递归函数，传递 &[&i32] 切片、索引 0 和空前缀
    print_heap_node_horizontal(display_slice, 0, "".to_string());

    println!(
        "\n... 堆的总大小为 1000，仅显示了前 {} 个节点以保持输出可读性。",
        display_slice.len()
    );
}
