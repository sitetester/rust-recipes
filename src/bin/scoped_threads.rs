// Automatically wait for all the spawned threads to wait (join), when the scope ends
fn main() {
    std::thread::scope(|scope| {
        scope.spawn(|| {
            println!("Hello from the spawned thread1");
        });

        scope.spawn(|| {
            println!("Hello from the spawned thread2");
        });
    });

    // No need to do something like this
    // std::thread::sleep(std::time::Duration::from_secs(5));

    // Or
    // handle1.join().unwrap();
    // handle2.join().unwrap();

    println!("Hello from the main thread");
}