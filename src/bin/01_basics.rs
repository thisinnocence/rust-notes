use std::fmt;

const MAX_RETRY: u32 = 3;

// `macro_rules!` 用来定义“声明式宏”。
// 宏不是函数调用，而是“按模式匹配后展开代码”（类似编译期模板展开）。
macro_rules! sum_i32 {
    // `($a:expr, $b:expr)` 是匹配规则：
    // - `$a` / `$b` 是宏参数名；
    // - `expr` 表示它们必须是表达式（如 1+2、x、foo()）。
    ($a:expr, $b:expr) => {
        // `=>` 左边是“匹配模式”，右边是“展开结果”。
        // 这里会把 `sum_i32!(10, 20)` 展开成 `10 + 20`。
        $a + $b
    };
}

// #[derive(Debug)] 会自动为类型生成 Debug trait 实现，
// 这样就可以用 {:?} 打印调试信息。
#[derive(Debug)]
enum Mode {
    Fast,
    Safe,
}

impl fmt::Display for Mode {
    // 这是在“为 Mode 实现 Display trait 要求的方法”。
    // 它看起来像普通函数，但语义上是 trait 的接口实现，不是随便起名的函数。
    // Display trait 规定：必须提供这个签名的 fmt 方法。
    // 人肉拆签名：
    // - `fn fmt(...) -> fmt::Result`
    //   `fmt` 是方法名，返回 `fmt::Result`（可理解为 `Result<(), fmt::Error>`）。
    // - `&self`
    //   当前对象的只读借用（这里是 `Mode`）。
    // - `f: &mut fmt::Formatter<'_>`
    //   `f` 是“格式化写入上下文”。
    //   `fmt::Formatter`：`std::fmt` 模块里的 `Formatter` 类型（`::` 是路径分隔符）。
    //   `<'_>`：给 `Formatter` 的生命周期参数，`'_` 表示“让编译器自动推断这个生命周期”。
    //   注意：这里的 `'_` 不是 Python 式“变量名下划线占位符”，而是“匿名生命周期参数”。
    //   `&mut`：可变借用，因为 `write!` 需要往 `f` 里持续写入内容。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self: &self
        // - 当前要被格式化输出的对象（这里就是 Mode::Fast / Mode::Safe）
        // f: &mut fmt::Formatter<'_>
        // - 格式化“写入器”，把最终文本写到这里，不是直接 println!
        // -> fmt::Result
        // - 返回格式化是否成功（本质是 Result<(), fmt::Error>）
        match self {
            // write! 是宏：把字符串写进 f，而不是写到 stdout。
            // 这里定义了 Display 的“用户可读”文本形式。
            Mode::Fast => write!(f, "fast"),
            Mode::Safe => write!(f, "safe"),
        }
    }
}

struct TaskRunner {
    // 字段语法是 `字段名: 类型`，和 C/C++ 的 `类型 变量名` 顺序不同。
    // 这个有点类似Golang
    name: String,
    mode: Mode,
}

impl TaskRunner {
    // `-> Self` 里的 `Self` 是类型层关键字，这里等价于 `TaskRunner`。
    fn new(name: &str, mode: Mode) -> Self {
        Self {
            name: name.to_string(),
            mode,
        }
    }

    // `&self` 里的 `self` 是值层接收者，表示当前实例的只读借用。
    fn run(&self, retries: u32) {
        println!("run name={}, mode={:?}, retries={retries}", self.name, self.mode);
    }
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for TaskRunner {
    fn describe(&self) -> String {
        format!("TaskRunner(name={}, mode={:?})", self.name, self.mode)
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn classify(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        75..=89 => "B",
        60..=74 => "C",
        _ => "D",
    }
}

fn maybe_timeout(mode: &Mode) -> Option<u64> {
    if matches!(mode, Mode::Fast) {
        Some(200)
    } else {
        None
    }
}

fn inspect_ref(v: &i32) {
    println!("inspect_ref value={v}, addr={:p}", v);
}

fn main() {
    // 关键字与变量：let / mut / const / shadowing
    let language = "rust";
    let mut score = 88;
    score += 7;
    let score = score; // shadowing: 绑定为新的不可变变量

    println!("language={language}, score={score}");
    println!("sum={}", add(3, 5));
    println!("sum via macro={}", sum_i32!(10, 20));

    // 控制流：if / for / while / loop / match
    let level = if score >= 90 { "A" } else { "B" };
    println!("level via if={level}");
    println!("level via match={}", classify(score));

    for i in 0..3 {
        println!("for i={i}");
    }

    for i in 0..10 {
        if i == 4 {
            println!("for break at i={i}");
            break;
        }
    }

    let mut n = 0;
    while n < 2 {
        println!("while n={n}");
        n += 1;
    }

    let mut m = 0;
    while m < 10 {
        m += 1;
        if m % 2 == 0 {
            continue;
        }
        if m > 5 {
            println!("while break at m={m}");
            break;
        }
        println!("while odd m={m}");
    }

    // loop 是无条件循环，语义上等价于 while true / for(;;)。
    // 这里演示 break 携带返回值，赋给 stop_at。
    let mut attempts = 0;
    let stop_at = loop {
        attempts += 1;
        if attempts >= MAX_RETRY {
            break attempts;
        }
    };
    println!("loop stop_at={stop_at}");

    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("break outer at x={x}, y={y}");
                break 'outer;
            }
        }
    }

    // "类" 对应：struct + impl
    let runner = TaskRunner::new("parser", Mode::Fast);
    runner.run(2);

    // "接口" 对应：trait
    println!("describe={}", runner.describe());
    // 这里的 `println!` / `format!` / `matches!` / `write!` 末尾 `!` 表示“宏调用”。
    println!("runner mode debug={:?}", runner.mode);
    println!("runner mode display={}", runner.mode);
    let is_fast = matches!(runner.mode, Mode::Fast);
    println!("is_fast={is_fast}, not_fast={}", !is_fast);

    // 引用与地址：&T 是借用（reference），不是 C 的裸指针。
    // 借用本身会指向某个地址，可用 {:p} 打印。
    let num = 42;
    let r = &num;
    println!("num={num}, r_addr={:p}", r);
    inspect_ref(r);
    println!("deref *r={}", *r);

    // 常见基础类型：Option
    match maybe_timeout(&runner.mode) {
        Some(ms) => println!("timeout={ms}ms"),
        None => println!("timeout=none"),
    }

    // 元组是 Rust 内建类型（这里是 (&str, i32)），可类比 C++ std::pair 语义。
    // Rust 访问方式是 .0 / .1，而不是 .first / .second。
    let pair = ("worker", 8);
    println!("tuple pair: role={}, count={}", pair.0, pair.1);

    let safe_runner = TaskRunner::new("indexer", Mode::Safe);
    println!("safe describe={}", safe_runner.describe());
}
