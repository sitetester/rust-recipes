use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
}

fn main() {
    COUNTER.with(|c| {
        *c.borrow_mut() = 3;
        println!("Counter: {}", c.borrow()); // Output: Counter: 1
    });

    std::thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() = 5;
            println!("Thread Counter: {}", c.borrow()); // Output: Thread Counter: 1
        });
    }).join().unwrap();

    COUNTER.with(|c| {
        println!("Main Counter: {}", c.borrow()); // Output: Main Counter: 1
    });
}