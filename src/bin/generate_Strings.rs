//

fn main() {
    // Using String::from
    let s1 = String::from("hello");
    println!("{}", s1);

    let s2: String = "hello".into();
    println!("{}", s2);

    // Using String::new and push_str
    let mut s3 = String::new();
    s3.push_str("hello");
    println!("{}", s3);

    // Using format!
    let s4 = format!("hello {}", "world");
    println!("{}", s4);

    // Direct assignment to a &str
    let s5: &str = "hello";
    println!("{}", s5);

    // Using to_string method
    let s6 = "hello".to_string();
    println!("{}", s6);

    let s7 = ['h', 'e', 'l', 'l', 'o'].iter().collect::<String>();
    println!("{}", s7);
}