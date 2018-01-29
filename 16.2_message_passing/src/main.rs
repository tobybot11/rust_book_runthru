use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    println!("Hello, world! messages will be passed");

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("thread1 -> hi"),
            String::from("thread1 -> from"),
            String::from("thread1 -> the"),
            String::from("thread! -> thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("t2 -> more"),
            String::from("t2 -> messages"),
            String::from("t2 -> for"),
            String::from("t2 -> you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
