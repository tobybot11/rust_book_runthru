use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!",i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
    }
}
