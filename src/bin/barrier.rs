use std::sync::{Arc, Barrier};
use std::thread;

const NUM_THREADS: usize = 3;

// A barrier enables multiple threads to synchronize the beginning of some computation
fn main() {
    let barrier = Arc::new(Barrier::new(NUM_THREADS));

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|id| {
            let barrier = Arc::clone(&barrier);

            thread::spawn(move || {
                println!("Thread {} before barrier", id);

                barrier.wait();

                println!("Thread {} after barrier", id);
            })
        })
        .collect();

    // Wait for all worker threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}