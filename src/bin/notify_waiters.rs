use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Create an Arc-wrapped Notify instance
    let notify = Arc::new(Notify::new());

    // Clone the Arc to share with the producer task
    let notify_producer = notify.clone();

    // Spawn the producer task
    tokio::spawn(async move {
        producer_task(notify_producer).await;
    });

    for i in 1..4 {
        let notify_clone = notify.clone();
        tokio::spawn(async move {
            consumer_task(i, notify_clone).await;
        });
    }

    sleep(Duration::from_secs(5)).await;
    println!("Done.");
}

async fn producer_task(notify: Arc<Notify>) {
    let interval = Duration::from_secs(2);

    // Simulate some work
    println!("Producer: performing work...");
    sleep(interval).await;

    // Notify the consumer task
    println!("Producer: notifying consumers...");
    notify.notify_waiters();
}

async fn consumer_task(n: u32, notify: Arc<Notify>) {
    // Wait for a notification from the producer task
    println!("Consumer{}: waiting for notification...", n);
    notify.notified().await;

    // Perform some action upon receiving the notification
    println!("Consumer{}: received notification, performing action...", n);
}