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

// GAT 示例：关联类型也可以带生命周期参数。
trait ChunkSource {
    type Chunk<'a>
    where
        Self: 'a;

    fn first_chunk<'a>(&'a self) -> Self::Chunk<'a>;
}

struct Packet {
    data: Vec<u8>,
}

impl ChunkSource for Packet {
    type Chunk<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn first_chunk<'a>(&'a self) -> Self::Chunk<'a> {
        &self.data[..self.data.len().min(4)]
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

    let packet = Packet {
        data: vec![1, 2, 3, 4, 5, 6],
    };
    println!("gat first chunk={:?}", packet.first_chunk());

    // 对象安全失败示例（仅注释，不参与编译）：
    // trait Bad { fn new() -> Self; }
    // let _: &dyn Bad; // 不成立：返回 Self 的 trait 方法通常不对象安全。
}
