const MAX_RETRY: u32 = 3;

// #[derive(Debug)] 会自动为类型生成 Debug trait 实现，
// 这样就可以用 {:?} 打印调试信息。
#[derive(Debug)]
enum Mode {
    Fast,
    Safe,
}

struct TaskRunner {
    // 字段语法是 `字段名: 类型`，和 C/C++ 的 `类型 变量名` 顺序不同。
    name: String,
    mode: Mode,
}

impl TaskRunner {
    fn new(name: &str, mode: Mode) -> Self {
        Self {
            name: name.to_string(),
            mode,
        }
    }

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

    // 控制流：if / for / while / loop / match
    let level = if score >= 90 { "A" } else { "B" };
    println!("level via if={level}");
    println!("level via match={}", classify(score));

    for i in 0..3 {
        println!("for i={i}");
    }

    let mut n = 0;
    while n < 2 {
        println!("while n={n}");
        n += 1;
    }

    let mut attempts = 0;
    let stop_at = loop {
        attempts += 1;
        if attempts >= MAX_RETRY {
            break attempts;
        }
    };
    println!("loop stop_at={stop_at}");

    // "类" 对应：struct + impl
    let runner = TaskRunner::new("parser", Mode::Fast);
    runner.run(2);

    // "接口" 对应：trait
    println!("describe={}", runner.describe());
    println!("runner mode debug={:?}", runner.mode);

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

    let safe_runner = TaskRunner::new("indexer", Mode::Safe);
    println!("safe describe={}", safe_runner.describe());
}
