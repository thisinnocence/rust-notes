// 格式说明（Rust 社区默认）：
// 1) 缩进是 4 空格（不是 2 空格）。
// 2) 行宽默认 100 列（由 rustfmt 控制换行）。
// 3) 看到两级/三级缩进很常见，社区通常通过“拆小函数、提前返回”
//    来降低嵌套，而不是改 formatter 规则。
mod net {
    pub mod tcp {
        pub struct Client {
            addr: String,
        }

        impl Client {
            pub fn new(addr: &str) -> Self {
                Self {
                    addr: addr.to_string(),
                }
            }

            pub fn addr(&self) -> &str {
                &self.addr
            }
        }
    }
}

use crate::net::tcp::Client;

fn main() {
    let c = Client::new("127.0.0.1:8080");
    println!("client addr={}", c.addr());
}
