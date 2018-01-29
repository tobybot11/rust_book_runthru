use std::thread;
use std::sync::mpsc;

fn main() {
    println!("Hello, world! messages will be passed");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
