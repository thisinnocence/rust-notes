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
