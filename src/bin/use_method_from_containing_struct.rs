// there is no struct inheritance in Rust, instead we can use composition to acheive similar goals.

use std::ops::Deref;

struct Logger {}
impl Logger {
    fn log(&self, text: &str) {
        println!("{}", text); // dummy log, update as needed
    }
}

struct User {
    logger: Logger,
}

impl Deref for User {
    type Target = Logger;
    fn deref(&self) -> &Logger {
        &self.logger
    }
}

// Use a method from containing struct (Deref polymorphism)
fn main() {
    let user = User { logger: Logger {} };
    user.log("Hello, world!");
}