enum Backend {
    Epoll,
    IoUring,
}

struct Config {
    worker_threads: usize,
    backend: Backend,
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Config {
    fn describe(&self) -> String {
        let backend = match self.backend {
            Backend::Epoll => "epoll",
            Backend::IoUring => "io_uring",
        };
        format!("threads={}, backend={backend}", self.worker_threads)
    }
}

// 关联类型示例：实现者决定输出类型
trait ValueType {
    type Output;
    fn value(&self) -> Self::Output;
}

impl ValueType for Config {
    type Output = usize;

    fn value(&self) -> Self::Output {
        self.worker_threads
    }
}

fn print_desc_static(item: &impl Describe) {
    println!("static dispatch: {}", item.describe());
}

fn print_desc_dynamic(item: &dyn Describe) {
    println!("dynamic dispatch: {}", item.describe());
}

fn main() {
    let c1 = Config {
        worker_threads: 4,
        backend: Backend::Epoll,
    };

    let c2 = Config {
        worker_threads: 8,
        backend: Backend::IoUring,
    };

    // impl Trait: 静态分发
    print_desc_static(&c1);
    // dyn Trait: 动态分发
    print_desc_dynamic(&c2);

    println!("associated type value={}", c1.value());
}
