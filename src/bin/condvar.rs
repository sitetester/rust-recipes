use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// `Condvar::new()` to conditionally wait for something during `thread::spawn`
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        sleep(Duration::from_secs(3));
        println!("Signaling start signal...");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("Waiting for start signal...");

    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("Started!");
}