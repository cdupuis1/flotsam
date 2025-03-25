
//
// This is a sample program which shows how a global sender/receiver can be
// declared and used safely.  Normally we would declare a send and receiver
// in main() but there are some case where we can't pass arguments to callback
// threads and we would want to reference channel between threads
//
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

// Note that we use different mutexes for the sender and receiver as the
// sender can be cloned and released quickly but the receiver needs to be
// held exclusively while we are using it
//
// Also note that for global use we use a Mutex to protect the Sender/Receiver
// and the Mutex is declared Lazy so it is not allocated until its first use
// Both of these are needed to keep the global data thread safe
static GLOBAL_CHANNEL: Lazy<Mutex<Option<Sender<String>>>> = Lazy::new(|| {
    Mutex::new(None)
});

// Store the receiver separately to avoid mutex deadlock
static GLOBAL_RECEIVER: Lazy<Mutex<Option<Receiver<String>>>> = Lazy::new(|| {
    Mutex::new(None)
});

// Initialize the global sender and receiver
async fn init_async() {
    // Declare the shared sender/receiver
    let (tx, rx) = mpsc::channel(100);
    
    // Store sender and receiver separately
    let mut channel_guard = GLOBAL_CHANNEL.lock().await;
    *channel_guard = Some(tx);
    drop(channel_guard);
    
    let mut rx_guard = GLOBAL_RECEIVER.lock().await;
    *rx_guard = Some(rx);
    
    println!("Async initialization complete");
}

// This is the receiver thread.  Note that since the global receiver is not
// cloneable, we must take/return the mutex outside the for loop
async fn start_worker() {
    println!("Worker task: Waiting for messages...");
    
    // Get the receiver once, outside the loop
    //
    // This prevents others from using it while we process it in the loop.  Also
    // we don't grab it in the loop as that would be unoptimal from a performance
    // point of view
    let rx = {
        let mut rx_guard = GLOBAL_RECEIVER.lock().await;
        rx_guard.take()
    };
    
    // receiver is set to rx to show that receiver is used to receive the message
    // and rx is used to manage if we have ownership of the mutex.  rx is actually
    // set equal to the result of the block withich is the contents of rx_guard.  Also
    // receiver is declared mut so we can make changes to it like receiving messages
    // without affecting the original rx variable
    if let Some(mut receiver) = rx {
        for _i in 1..=5 {
            if let Some(message) = receiver.recv().await {
                println!("Worker task: Received: {}", message);
            } else {
                println!("Channel closed");
                break;
            }
        }
        
        // Put the receiver back when done outside the loop
        let mut rx_guard = GLOBAL_RECEIVER.lock().await;
        *rx_guard = Some(receiver);
    } else {
        println!("Channel not initialized!");
    }
    
    println!("Worker task: Completed processing messages.");
}

// This threads sends a string to the receiver and then waits 500ms before sending
// the next
//
// @param message The string to send to the receiver
async fn start_sender(message: &str) {
    for i in 1..=5 {
        // Clone the sender so we don't need to remove it from GLOBAL_CHANNEL
        let tx_clone = {
            let channel_guard = GLOBAL_CHANNEL.lock().await;
            match &*channel_guard {
                Some(tx) => Some(tx.clone()),
                None => None,
            }
        };
        
        // *** Lock is automatically released here as the local channel guard
        //     variable goes out of scope.  Note that we can clone and Sender
        //     and not a receiver so this is why we don't have to get and release
        //     the lock outside the for loop ***
        if let Some(tx) = tx_clone {
            match tx.send(message.to_string()).await {
                Ok(_) => println!("Sent message {}: {}", i, message),
                Err(e) => println!("Failed to send: {}", e),
            }
        } else {
            println!("Channel not initialized!");
        }
        
        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    // Initialize global sender/receiver
    init_async().await;

    // Spawn the worker task
    let handle1 = tokio::spawn(start_worker());

    // Spawn the sender task
    let handle2 = tokio::spawn(start_sender("Hello from sender"));

    // Wait for both tasks to complete
    let result = tokio::join!(handle1, handle2);

    // Note, the match here is a little odd as it returns a tuple where each memeber of
    // the tuple is Result<(), JoinError> since we are joining the results of two different
    // processes
    match result {
        (Ok(()), Ok(())) => println!("All tasks completed"),
        (Err(e1), _) => println!("Error in first task: {}", e1),
        (_, Err(e2)) => println!("Error in second task: {}", e2),
    }
}
