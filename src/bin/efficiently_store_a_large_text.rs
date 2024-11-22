use std::mem::size_of_val;
use std::rc::Rc;

struct Article1<'a> {
    contents: &'a str,
}

struct Article2 {
    contents: Rc<String>,
}

// How can I efficiently store a large text in two separate structs ?
fn main() {
    let text = "some very very LONG  LONG  LONG  LONG text";

    // option1: store a reference (but then check println output)
    let a1 = Article1 {
        contents: &text
    };
    let a2 = Article1 {
        contents: &text
    };
    println!("{}", size_of_val(&a1.contents));
    println!("{}", size_of_val(&a2.contents));

    // option2: use Rc
    let rc = Rc::new(text.to_string());

    let a3 = Article2 { contents: rc.clone() };
    let a4 = Article2 { contents: rc.clone() };
    println!("{}", size_of_val(&a3.contents));
    println!("{}", size_of_val(&a4.contents));
}