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

fn print_desc(item: &impl Describe) {
    println!("{}", item.describe());
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

    print_desc(&c1);
    print_desc(&c2);
}
