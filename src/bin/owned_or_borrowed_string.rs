use std::borrow::{Borrow, Cow};

fn print_borrowed<T: Borrow<str>>(s: T) {
    println!("Borrowed str: {}", s.borrow());
}

// A function taking either owned or borrowed string
fn main() {
    let string = String::from("hello");
    let str_slice: &str = "world";

    // Borrow String as &str (for both cases)
    print_borrowed(string.clone());
    print_borrowed(str_slice);

    // Can pass either owned or borrowed str
    print_it(Cow::Owned(string));
    print_it(Cow::Borrowed(str_slice));
}

fn print_it(s: Cow<str>) {
    println!("Owned or Borrowed: {}", s);
}