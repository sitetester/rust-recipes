use std::fmt;

struct User {
    name: String,
    age: u8,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

fn print_string(input: String) {
    println!("{input}");
}

// `to_string()` for custom type
fn main() {
    let alex = User {
        name: "Alex".to_string(),
        age: 25,
    };

    // `User` is custom struct, but we are able to call `to_string()`
    // Also note that `print_string` expects a `String`
    print_string(alex.to_string());
}