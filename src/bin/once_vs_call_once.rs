use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();
static mut VAL: Option<i32> = None;

fn init() {
    println!("init called only once");

    static mut COUNTER: i32 = 0;
    unsafe {
        // Assign a value to the mutable static variable
        COUNTER += 1;
        VAL = Some(COUNTER);
    }
}


fn call_once_demo() {
    // initialize from main
    INIT.call_once(init);

    // Spawn a separate thread to mutate VAL
    // will have no effect, since it was initialized only once from `main`
    let handle = thread::spawn(|| {
        INIT.call_once(init);
    });
    handle.join().unwrap();

    // Access the value safely in the main thread
    unsafe {
        // Print the value of VAL
        println!("VAL: {:?}", VAL);
    }
}

// `Once` vs `OnceLock` to initialise some value once per application (irrespective of no of threads calling it)
fn main() {
    call_once_demo();
    once_lock_demo();
}

fn once_lock_demo() {
    use std::sync::OnceLock;

    // Declare a static OnceLock that can hold a String
    static CELL: OnceLock<String> = OnceLock::new();

    // Create a value to set in the OnceLock
    let value = "Hello, World!".to_string();

    // Attempt to set the value in the OnceLock
    // This will only succeed if cell is not already set
    if CELL.set(value).is_ok() {
        println!("OnceLock was successfully set.");
    } else {
        println!("OnceLock was already set.");
    }

    thread::spawn(|| {
        // In a different thread, attempt to set the value
        if CELL.set("Hello, Rust!".to_string()).is_ok() {
            println!("OnceLock was successfully set in a different thread.");
        } else {
            println!("OnceLock was already set in a different thread.");
        }
    }).join().unwrap();
}
