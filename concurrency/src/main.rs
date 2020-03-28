use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    println!("Hello, world!");
    spawn_threads();
    moving_values();
    channelling();
    shared_state();
}

fn spawn_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn moving_values() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channelling() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    thread::spawn(move || {
        let val = String::from("hi1");
        tx1.send(val).unwrap();
    });

    for received in rx {
        println!("Got {}", received)
    }
}

fn shared_state() {

}
