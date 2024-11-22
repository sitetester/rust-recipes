fn main() {
    let name1 = String::from("Alex");
    let v1 = (1..=3).collect::<Vec<i32>>();

    // if we don't use `move` here, we get following errors

    // `error[E0373]: closure may outlive the current function, but it borrows `v1`, which is owned by the current function`
    // `help: to force the closure to take ownership of `v1` (and any other referenced variables), use the `move` keyword`
    // `note: function requires argument type to outlive `'static``
    let handle1 = std::thread::spawn(move || {
        println!("Hello, {}!", name1);
        for i in &v1 {
            println!("Value: {}", i);
        }
    });
    handle1.join().unwrap();

    // alternatively, we could use `scoped threads` which are guaranteed to finish before the scope returns
    let name2 = String::from("John");
    let v2 = (4..=6).collect::<Vec<i32>>();
    std::thread::scope(|_scope| {
        println!("Hello, {}!", name2);
        for i in &v2 {
            println!("Value: {}", i);
        }
    });

    println!("Hello from the main thread");
}