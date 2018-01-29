use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!",i);
        }
    });

    handle.join();

    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
    }

    // handle.join();

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
       println!("Here's a vector: {:?}", v) ;
    });

    // drop(v);

    handle.join();
}
