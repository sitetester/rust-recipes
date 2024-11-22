// Rust allows us to implement a trait for all types that satisfy a certain condition

use std::fmt::Debug;

trait Debuggable {
    fn debug(&self)
    // Add a trait bound here because of the formatting specifier `{:?}`
    // Note: it's Self(S capital), since it refers to the implementation type (T)
    where
        Self: Debug,
    {
        println!("{:?}", self);
    }
}

// T must implement `Debug` trait
// Then we can make it auto implement `Debuggable` trait
impl<T: Debug> Debuggable for T {}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
}

// Coherence in trait implementations
// It ensures that there won’t be multiple conflicting implementations of the same trait for a given type,
fn main() {
    let number = 12;
    number.debug();

    let name = "Alex";
    name.debug();

    let f = 1.2;
    f.debug();

    let user: User = User { name: "Alex".to_string() };
    user.debug();


    // similar example
    example2();
}

fn example2() {
    use std::fmt;

    trait Loggable {
        fn log(&self) -> String;
    }

    impl fmt::Display for dyn Loggable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.log())
        }
    }

    struct Error {
        message: String,
    }

    impl Loggable for Error {
        fn log(&self) -> String {
            format!("Error: {}", self.message)
        }
    }

    let error = Error {
        message: String::from("An error occurred."),
    };

    let d: Box<dyn Loggable> = Box::new(error);
    println!("{}", d);
}