// use rust_reactor::runtime::Runtime;

// fn main() {
//     let runtime = Runtime::new();
//     runtime.run();
// }

// use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{Sender, Receiver};
// use std::thread;
// use std::time::Duration;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn test_multi_threads() {
    // let queue = Arc::new(Mutex::new(VecDeque::new()));
    // let q = queue.clone();

    // let _event_loop = thread::spawn(move || {
    // 	loop {
    // 	    match q.lock().unwrap().pop_front() {
    // 		Some(val) => println!("pop value: {}", val),
    // 		None => {},
    // 	    }
    // 	    thread::sleep(Duration::from_secs(1));
    // 	}
    // });

    // let mut val = 0;
    // loop {
    // 	queue.lock().unwrap().push_back(val);
    // 	val += 1;
    // 	thread::sleep(Duration::from_secs(1));
    // }
}

const SENDERS: i32 = 50;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut sender_threads = Vec::new();

    for id in 0..SENDERS {
	let alt_tx = tx.clone();
	let sender = thread::spawn(move || {
	    for cnt in 0..5 {
		let mut message = cnt.to_string();
		message.push_str(": ");
		message.push_str(&id.to_string());

		alt_tx.send(message).unwrap();
		thread::sleep(Duration::from_secs(1));
	    }
	});
	sender_threads.push(sender);
    }

    for _ in 0..SENDERS {
	let received_value = rx.recv();
	println!("received {:?}", received_value);
    }

    for sender in sender_threads {
	sender.join().expect("Error: Thread is stopped for failer.");
    }
}
