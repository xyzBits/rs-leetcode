use std::cell::{Ref, RefCell};
use std::rc::Rc;

/// pre next 可有可无，所以是 option
/// prev next 是对其他节点的引用，所以没有对应节点的所有权，所用 Rc 共享所有权，Rc 表示不可变的 shared_ptr
/// 双向链接的节点变动会牵涉 prev 和 next 字段的变动，但 Rc 不可变的，就算 node 设置为 mut 也不行，所以要使用 RefCell 包装一下变成可变的，
///
struct Node<T> {
    ele: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(ele: T) -> Self {
        Self {
            ele,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        // 创建 node，不需要设置为 mut
        // 因为 prev next 字段由 RefCell 包裹
        // 所以 prev next 都是可变的
        let node = Rc::new(RefCell::new(Node::new(elem)));

        // 这里要特别注意一下
        // match 会导致所有权转移
        // tail 是属于 self 的字段，rust 不允许所有权被转移走
        // 这是使用 option 的 take 方法，把内部值转移走，而 self.tail 变为 none
        // take 会消费掉 Option 中包裹的值，也就是说只能调用一次，后面的调用得到的都是 none
        match self.tail.take() {
            Some(tail) => {
                // borrow_mut 是 RefCell 的方法，让内部的值可变
                // Rc 没有实现 Copy trait，但是实现了 Clone，不过需要手动调用一下该方法
                // 代码逻辑变得比较简单了，如果 tail 存在，则往后追加节点，并把节点链接起来
                // 下面这两行调换，应该也可以
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(tail);
                self.tail = Some(node);
            }

            None => {
                // 如果 self.tail 是 none 表示第一次 push，则更新一下 self.head
                // 因为双向链表只有一个值，self.head self.tail 应该是一样的
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // take 方法见 push_back 方法中的注释
        // 因为 pop_back 方法有返回值，采用 Option::map 的方式
        // 比较自然，如果 self.tail 是 None 就直接返回 None
        // tail 的所有权还在，但是 Option 中的值成为 None 了
        self.tail.take().map(|node| {
            // 判断最后一个节点有没有 prev 节点，
            // 如果有则断开，如果没有则把 self.tail self.head 一起变成 none
            match node.borrow_mut().prev.take() {
                Some(prev) => {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }

                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            // 这里比较关键，
            // Node 是 Rc 类型，表示智能指针，共享了所有权
            // 但是 pop 则表示把 node 从双向链表中删除，即所有权转移走
            // 我们又不知道有没有其他地方共享了所有权，所以使用 Rc::try_unwrap
            // 这个 try 很关键，因为编译器不知道，所以需要支时判断
            // 中间的 ok 函数表示 把 result 类型转换成 Option 类型
            // into_inner() 是将 RefCell<T> 转换成 T ,最终所有权被释放出来 了
            Rc::try_unwrap(node).ok().unwrap().into_inner().ele
        })
    }

    pub fn peek_back(&self) -> Option<Ref<'_, T>> {
        self.tail.as_ref().map(|node| {
            // 由于 node.borrow() 返回的是 Ref<Node<T>
            // 如果 peek_back 直接返回 Ref<Node<T>>，则把内部的细节 Node 类型
            // 暴露给用户，所以需要把内部细节屏蔽掉
            // 使用 Ref::map 可以把内部字段映射出来
            Ref::map(node.borrow(), |node| &node.ele)
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

struct LRUCache {
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        todo!()
    }

    fn get(&self, key: i32) -> i32 {
        todo!()
    }

    fn pu(&self, key: i32, value: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    #[test]
    fn test_match() {
        let input = "hello".to_string();

        // match 会转移所有权
        match input {
            data => {
                println!("{}", data);
            }
        }

        // println!("{}", input);

        // if let 不会转移所有权
        let input = "hello".to_string();
        if let data = "hello".to_string() {
            println!("{}", data);
        }

        println!("{}", input);

        let input = Some("hello world".to_string());
        match input {
            Some(ref data) => {
                println!("{}", data);
            }
            None => {
                println!("sha bi");
            }
        }

        println!("{:?}", input);

        let input = Some("hello rust".to_string());

        match input.as_ref() {
            Some(data) => {
                println!("{}", data);
            }
            None => {}
        }

        println!("{:?}", input);
    }

    #[test]
    fn test_option_take() {
        let mut data = Some("hello");

        let mut data: Option<&str> = None;

        match data.take() {
            None => {
                println!("nothing");
            }
            Some(input) => {
                println!("{}", input);
            }
        }

        println!("{:?}", data);

        data.take();
    }

    #[test]
    fn test_refcell() {
        let input = RefCell::new("hello".to_string());
        let data = input.into_inner();

        // Value used after being moved
        // println!("{:?}", input);

        let input = Rc::new("hello".to_string());

        // 所有权被转移走了
        let result = Rc::try_unwrap(input);

        // Value used after being moved
        // println!("{}", input);

        let data = result.ok().unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_ref_map() {
        let input = RefCell::new((5, 'b'));
        let b1 = input.borrow();
        let b2 = Ref::map(b1, |t| &t.0);

        assert_eq!(*b2, 5);
    }

    #[test]
    fn test_ref() {
        let input = RefCell::new(vec![1, 2, 3]);
        // let input_ref = input.borrow();
        //
        // println!("{:?}", input_ref);

        let mut mut_ref_input = input.borrow_mut();
        mut_ref_input.push(4);

        println!("{:?}", mut_ref_input);

        println!("{:?}", input);
    }
}
