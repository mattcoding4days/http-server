pub struct Server {
    addr: String,
}

impl Server {
    // associated functions are known as static functions in C++
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Running server on {}", self.addr);
    }
}
