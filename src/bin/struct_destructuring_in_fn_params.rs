
#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
    height: u8,
}

// Structure destructuring in function params:
fn main() {
    // `..` here means ignore the remaining fields
    fn destructure_demo(Person { name, age, .. }: &Person) {
        println!("{name} is {age} years old.");
    }

    let p = Person {
        name: "Alex".to_string(),
        age: 30,
        height: 174,
    };

    destructure_demo(&p);
}