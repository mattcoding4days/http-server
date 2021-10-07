fn main() {
    //let server = Server::new("127.0.0.1:8080".to_string());

    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];

    dbg!(&string);
    dbg!(string_slice);
}

struct Server {
    addr: String,
}

impl Server {
    // associated functions are known as static functions in C++
    fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    fn run(self) {
        println!("Running server on {}", self.addr);
    }
}
