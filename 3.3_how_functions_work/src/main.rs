// 3.3 functions and 3.4 comments

// Hello, world

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.

fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x2 = 5;
    println!("The value of x2 is: {}", x2);
    let y2 = {
        let x2 = 3;
        x2 + 1
    };
    println!("The value of x2 and y2 is {} : {}", x2, y2);

    let x3 = five();
    println!("the value of x3 is : {}", x3);

    let x4 = plus_one(5);

    println!("the value of x4 is {}", x4);
   
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn five() -> i32 {
    5 // five is 5
}

fn plus_one(x: i32) -> i32 {
    // i'm adding one
    x + 1
}
