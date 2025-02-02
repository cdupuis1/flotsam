//
// Small program to show how to send a signal from one thread to another
//
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Signal {
    data: String,
}

// Declare the global, static variables
static SIGNAL: Mutex<Option<Signal>> = Mutex::new(None);
static CONDVAR: Condvar = Condvar::new();

// Loops waiting for signal to be assigned and then printing the
// value
fn receiver()
{
    loop {
        // Assign a local variable to the global mutex
        let mut signal = SIGNAL.lock().unwrap();

        // Do nothing until signal is assigned something in the sender thread
        while signal.is_none() {
            // Releases the mutex and puts this thread to sleep.  We will not
            // proceed until the mutex is reacquired so that the is never
            // any concurrent access to the data in signal
            signal = CONDVAR.wait(signal).unwrap();
        }

        // Note, use .as_ref() to get the underlying value of signal
        // with grabbing a reference and having to borrow
        println!("Signal received: {}", signal.as_ref().unwrap().data);

        // Reset the signal so when the loop comes back around we wait
        // for the sender to set it again
        *signal = None;
    }
}


// Sends a string with a monotonically increasing index to the receiver
fn sender()
{
    let mut index = 0;

    loop {
        // Sleep so we only signal the receiver every second
        thread::sleep(Duration::from_secs(1));

        // Grab the mutex so we can assign it and then wake the receiver thread.
        // The shared mutex is what protects the data within signal
        let mut signal = SIGNAL.lock().unwrap();
        *signal = Some(Signal {
            data: format!("index={}", index),
        });

        println!("Signal sent");

        CONDVAR.notify_one();
        index += 1;
    }
}

fn main() {
    // Spawn the receiver thread FIRST
    let receiver_handle = thread::spawn(move || receiver());

    // Spawn the sender thread
    let sender_handle = thread::spawn(move || sender());

    // Wait for threads to exit
    receiver_handle.join().unwrap();
    sender_handle.join().unwrap();
}