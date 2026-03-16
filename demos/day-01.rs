//! Day 1 练习脚手架
//!
//! 运行:
//! cargo run --bin day-01
//!
//! 目标:
//! - 先保证能读懂每个 TODO 的意图
//! - 再自己补实现
//! - 每补完一段就运行一次确认输出

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

const RANGE_START: i32 = 1;
const RANGE_END: i32 = 5;

static APP_NAME: &str = "day-01-exercises";
static SUM_CALLS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct DayOneReport {
    sum_result: i32,
    array_total: u32,
    vec_len: usize,
    boxed_value: i32,
    rc_count: usize,
}

impl DayOneReport {
    fn print_summary(&self) {
        println!("sum_result={}", self.sum_result);
        println!("array_total={}", self.array_total);
        println!("vec_len={}", self.vec_len);
        println!("boxed_value={}", self.boxed_value);
        println!("rc_count={}", self.rc_count);
    }
}

fn sum_range(start: i32, end: i32) -> i32 {
    SUM_CALLS.fetch_add(1, Ordering::Relaxed);

    // TODO(day1):
    // 用 for + 闭区间 start..=end 自己实现求和。
    // 现在先给占位实现，保证程序可运行。
    let mut sum = 0;
    for i in start..=end {
        // 闭区间， [start, end] 因为是 ..= 有 =
        sum += i;
    }
    sum
}

fn array_and_slice_demo() -> u32 {
    let mut numbers: [u32; 5] = [10, 20, 30, 40, 50];

    // TODO(day1):
    // 1. 改成 `let mut numbers`
    // 2. 修改其中一个元素
    // 3. 取一个切片，比如 &numbers[1..4]
    // 4. 打印数组、切片、长度
    //
    // 提示:
    // - 数组类型形如 [T; N]
    // - 切片类型形如 &[T]
    numbers[0] = 99; // 因为是 mut numbers，所以可以修改元素
    let middle: &[u32] = &numbers[1..4]; // 切片, 索引 [1, 4) 左闭右开区
    println!("numbers={numbers:?}");
    println!("middle={middle:?}");
    println!("numbers len={}", numbers.len());
    println!("middle len={}", middle.len());

    // TODO(day1):
    // 用迭代器或循环算出数组元素和。
    let mut total = 0;
    for i in numbers {
        total += i;
    }
    total
}

fn vec_clone_demo() -> usize {
    let mut source = vec![1_u32, 2, 3];

    // TODO(day1):
    // 1. clone 一份新的 Vec
    // 2. 给原 Vec push 一个新元素
    // 3. 打印原 Vec 和 clone 后的 Vec，观察它们互不影响
    let copied = source.clone(); // clone 是深拷贝，复制了 Vec 里的数据，所以修改 source 不会影响 copied
    source.push(4);

    println!("vec source={source:?}");
    println!("vec copied={copied:?}");

    // TODO(day1):
    // 返回原 Vec 的长度。
    source.len()
}

fn box_demo() -> i32 {
    // TODO(day1):
    // 把一个整数放到 Box<i32> 里，然后打印:
    // - Box 本身的 Debug 输出
    // - 解引用后的值 `*boxed`
    //
    // 你可以把下面这行替换成你自己的值。
    let boxed = Box::new(3); // box堆上的数据

    println!("boxed={boxed:?}");
    println!("boxed value={}", *boxed);

    *boxed
}

fn rc_demo() -> usize {
    let shared = Rc::new(String::from("rust"));

    // TODO(day1):
    // 1. 用 Rc::clone 再克隆两份
    // 2. 分别打印 strong_count
    // 3. drop 掉其中一个，再观察 strong_count 的变化
    let shared_2 = Rc::clone(&shared);
    println!("count = {}", Rc::strong_count(&shared));

    let shared_3 = Rc::clone(&shared);
    println!("count = {}", Rc::strong_count(&shared));

    drop(shared_2);
    println!("count = {}", Rc::strong_count(&shared));

    println!("shared value={}", shared.as_str());
    println!("shared_3 value={}", shared_3.as_str());

    Rc::strong_count(&shared)
}

fn build_report() -> DayOneReport {
    DayOneReport {
        sum_result: sum_range(RANGE_START, RANGE_END),
        array_total: array_and_slice_demo(),
        vec_len: vec_clone_demo(),
        boxed_value: box_demo(),
        rc_count: rc_demo(),
    }
}

fn print_day1_checklist() {
    println!();
    println!("== Day 1 checklist ==");
    println!("[ ] 我能解释 [T; N] 和 &[T] 的区别");
    println!("[ ] 我能解释切片为什么是 borrow view");
    println!("[ ] 我能解释 Vec::clone 为什么是深拷贝");
    println!("[ ] 我能解释 Box<T> 为什么常用于堆分配");
    println!("[ ] 我能解释 Rc<T> 为什么适合单线程共享所有权");
    println!("[ ] 我能自己把占位实现替换成真实逻辑");
}

fn main() {
    println!("app={APP_NAME}");

    let report = build_report();
    println!("report={report:#?}");
    report.print_summary();
    println!("sum_called={} time(s)", SUM_CALLS.load(Ordering::Relaxed));

    print_day1_checklist();
}
