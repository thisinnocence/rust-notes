//! demos/main.rs
//! 全局变量示例
//!
//! 运行方式:
//! 在 git 仓库根目录执行
//! cargo run --bin demo

use std::sync::atomic::{AtomicUsize, Ordering};
// Rc = Reference Counted, 单线程引用计数智能指针
use std::rc::Rc;

// 编译期常量, 不占用可变全局状态
const RANGE_START: i32 = 1;
const RANGE_END: i32 = 100;

// 只读全局变量
static APP_NAME: &str = "demo-global-var";

// 可变全局推荐用法之一, 用原子类型保证并发安全
static SUM_CALLS: AtomicUsize = AtomicUsize::new(0);

fn sum(start: i32, end: i32) -> i32 {
    SUM_CALLS.fetch_add(1, Ordering::Relaxed);
    let mut total = 0;
    // 迭代闭区间 [start, end], ..= 表示包含结束值
    for i in start..=end {
        total += i;
    }
    // 尾表达式返回: 最后一行无分号会作为返回值(等价于 return total;)
    // 常用写法: 更简洁且和 Rust 的表达式风格一致, return 通常留给提前返回
    total
}

fn array_demo() {
    // 固定长度数组, 显式类型标注是 [u32; 5], 这里的 ; 表示"元素类型;长度"
    let mut nums: [u32; 5] = [1, 2, 3, 4, 5];
    nums[0] = 10;

    // 切片类型是 &[u32], 1 和 4 都是索引边界, start..end 表示 [start, end)
    // 这里是不可变借用: let 默认不可变绑定, & 默认不可变借用, 不会产生 copy
    let mid = &nums[1..4];
    let total: u32 = nums.iter().sum();

    println!("array={nums:?}");
    println!("array_len={}", nums.len());
    println!("first={} last={}", nums[0], nums[nums.len() - 1]);
    println!("slice_mid={mid:?}");
    println!("array_sum={total}");
}

fn dynamic_array_demo() {
    // Rust 动态数组是 Vec, 可类比 C++ STL 的 std::vector(底层在堆上, 可扩容)
    let mut heap_arr: Vec<u32> = Vec::with_capacity(5);
    for i in 1..=5 {
        heap_arr.push(i);
    }

    // clone 会做深拷贝, 得到另一份独立堆内存, 可类比手工复制数组
    let copied_arr = heap_arr.clone();

    println!("heap_arr={heap_arr:?}");
    println!("copied_arr={copied_arr:?}");

    // 显式 drop 可类比提前 delete[], 不写也会在作用域结束时自动释放
    drop(heap_arr);
    println!("after drop heap_arr, copied_arr still alive={copied_arr:?}");
}

fn shared_ownership_demo() {
    // Rc<T> 表示单线程共享所有权, clone 只增加引用计数, 不会深拷贝底层数据
    let s1: Rc<String> = Rc::new(String::from("shared-text"));
    let s2 = Rc::clone(&s1);
    let s3 = Rc::clone(&s1);

    println!("shared_value={}", s1.as_str());
    println!("rc_count_after_clone={}", Rc::strong_count(&s1));

    drop(s2);
    println!("rc_count_after_drop_s2={}", Rc::strong_count(&s1));

    drop(s3);
    println!("rc_count_after_drop_s3={}", Rc::strong_count(&s1));
}

fn main() {
    let sum = sum(RANGE_START, RANGE_END);
    println!("app={APP_NAME}");
    println!("{sum}");
    println!("sum_called={} time(s)", SUM_CALLS.load(Ordering::Relaxed));
    array_demo();
    dynamic_array_demo();
    shared_ownership_demo();
}
