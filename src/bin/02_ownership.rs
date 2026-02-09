use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    val: i32,
    // `next` 不是 C 的裸指针字段，而是“可空 + 拥有所有权”的安全指针封装。
    // `Some(Box<Node>)` 表示有后继节点，`None` 表示链表到尾。
    next: Option<Box<Node>>,
}

#[derive(Debug, Default)]
struct SinglyList {
    head: Option<Box<Node>>,
}

impl SinglyList {
    fn push_front(&mut self, val: i32) {
        // 语法技巧：
        // 1) `val,` 是字段初始化简写，等价于 `val: val`。
        // 2) `self.head.take()` 会把旧 head 搬出并把原位设为 `None`，
        //    这样可在不克隆的情况下把“旧链表头”接到新节点的 next。
        let new_head = Box::new(Node {
            val,
            next: self.head.take(),
        });
        // 新节点成为链表头：`Some(new_head)` 表示当前 head 现在“有值”。
        self.head = Some(new_head);
    }

    fn pop_front(&mut self) -> Option<i32> {
        // take() 把 head 里的值“搬出来”，原位置变成 None，便于安全地重连链表。
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.val
        })
    }
}

// `s: String` 会接收所有权（move），不是参数副本复制。
// 调用方把 String 传进来后，原变量默认不可再使用（除非先 clone）。
fn takes_ownership(s: String) {
    println!("owned: {s}");
}

fn borrow_len(s: &str) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

// C++ 常见“我指向你、你指向我”的对象关系，在 Rust 里通常这样建模：
// - 共享所有权：Rc<T>
// - 需要内部可变：RefCell<T>
// - 反向引用避免循环：Weak<T>
#[derive(Debug)]
struct Parent {
    name: String,
    // 近似 C++：`std::vector<std::shared_ptr<Child>>`
    // 再叠加 `RefCell` 提供受控的“内部可变”。
    children: Vec<Rc<RefCell<Child>>>,
}

#[derive(Debug)]
struct Child {
    name: String,
    // 反向指针用 Weak，避免 Parent <-> Child 都是强引用导致泄漏。
    parent: Weak<RefCell<Parent>>,
}

fn bidirectional_demo() {
    let parent = Rc::new(RefCell::new(Parent {
        name: "P".to_string(),
        children: Vec::new(),
    }));

    let child = Rc::new(RefCell::new(Child {
        name: "C".to_string(),
        parent: Weak::new(),
    }));

    // Parent 强引用 child
    parent.borrow_mut().children.push(Rc::clone(&child));
    // Child 弱引用 parent
    child.borrow_mut().parent = Rc::downgrade(&parent);

    println!(
        "counts: parent strong={}, weak={}",
        Rc::strong_count(&parent),
        Rc::weak_count(&parent)
    );

    // 从 Weak 升级回 Rc（如果父对象还活着）
    if let Some(upgraded_parent) = child.borrow().parent.upgrade() {
        println!(
            "child={} -> parent={}",
            child.borrow().name,
            upgraded_parent.borrow().name
        );
    }
}

fn singly_list_demo() {
    // 经典 ownership 练习：Option<Box<Node>>
    // - Box: 唯一所有权 + 堆分配节点
    // - Option: 显式表达“有 next / 无 next”
    let mut list = SinglyList::default();
    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    println!("list pop1={:?}", list.pop_front()); // Some(30)
    println!("list pop2={:?}", list.pop_front()); // Some(20)
    println!("list pop3={:?}", list.pop_front()); // Some(10)
    println!("list pop4={:?}", list.pop_front()); // None
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move, s1 不再可用

    takes_ownership(s2);

    let mut msg = String::from("hello");
    let len = borrow_len(&msg);
    println!("msg={msg}, len={len}");

    append_world(&mut msg);
    println!("after mut borrow: {msg}");

    // 规则提示：
    // 1) 同一时刻，要么多个不可变借用，要么一个可变借用。
    // 2) 引用必须始终有效，编译期检查。

    bidirectional_demo();
    singly_list_demo();
}
