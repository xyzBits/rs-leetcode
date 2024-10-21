//! Rust 提供 *const T *mut T 两种类型的指针，
//! 因为这两种指针和 C 语言的指针十分相似，所以也叫其原始指针 raw pointer
//! 原始指针有以下特点：
//! 1。 并不保证指向合法的内存，比如很有可能指向一个空指针
//! 2。 并不像智能指针那样自动清理内存，需要像 C 语言那样手动管理内存
//! 3。 没有生命周期概念，也就是说，编译器不会对其提供借用检查
//! 4。 不能保障线程安全
//!

pub struct Node<T> {
    ele: T,
    prev: *const Node<T>,
    next: *const Node<T>,
}

impl<T> Node<T> {
    pub fn new(ele: T) -> Self {
        Self {
            ele,
            prev: std::ptr::null(),
            next: std::ptr::null(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unsafe_lru::Node;

    #[test]
    fn it_works() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);

        let n1 = &mut node1 as *mut Node<i32>;
        let n2 = &mut node2 as *mut Node<i32>;

        node1.next = &node2 as *const Node<i32>;
        node2.prev = &node1 as *const Node<i32>;

        let node = unsafe { &*node1.next };

        let node3 = Box::new(Node::new(3));

        let node4 = Box::leak(node3) as *const Node<i32>;
    }
}
