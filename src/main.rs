#![feature(await_macro, async_await, futures_api)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tokio;

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    key: &'a [u8],
    value: &'a [u8],
    headers: HashMap<&'a [u8], &'a [u8]>,
    timestamp: u128,
}

trait Producer<T: serde::Serialize> {
    fn produce(self, t: &T);
}

trait Consumer<T> {
    fn consume(self, t: &T);
}

struct BinProducer {
    ln: TcpStream,
}

impl BinProducer {
    fn new(addr: &String) -> BinProducer {
        let stream = TcpStream::connect(addr).unwrap();
        BinProducer { ln: stream }
    }
}

impl<T: serde::Serialize> Producer<T> for BinProducer {
    fn produce(mut self, t: &T) {
        let s = serde_json::to_string(t).expect("msg: &str");
        self.ln.write_all(s.as_bytes()).expect("msg: &str");
    }
}

fn main() {
    let x = Message {
        key: &[1],
        value: &[1],
        timestamp: 1,
        headers: HashMap::new(),
    };
    let p = BinProducer::new(&String::from("localhost:8888"));
    p.produce(&x);
}
