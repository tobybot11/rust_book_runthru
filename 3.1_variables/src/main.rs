fn main() {
    // 3.1 variables
    let mut x = 5;
    println!("The value of x is: {}",x );
    x = 6;
    println!("The value of x is: {}",x );

    let y = 5;
    println!("The value of y is: {}",y );
    let y = y + 1;
    println!("The value of y is: {}",y );
    let y = y * 2;
    println!("The value of y is: {}",y );

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    //3.2 data types
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guesss is: {}", guess);

    // floating point

    let xf = 2.2; // f64
    let yf : f32 = 3.3; //f32
    println!("The value of xf and yf is: {:?} and {:?} respectively ", xf, yf );

    // Numeric Operations

    let sum = 5 + 10;
    println!("sum is {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference is {}", difference);
    let product = 4 * 30;
    println!("product is {}", product);
    let quotient = 56.7 / 32.2;
    println!("quotient is {}", quotient);
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);

    // Boolean Type
    let t = true;
    let f: bool = false;
    println!("t is {} and f is {}", t, f);

    // Char
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("c is {} z is {} and heart eyed cat is {}", c, z, heart_eyed_cat);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4,1);
    println!("tup is {:?}", tup);
    let (x, y, z) = tup;
    println!("The value of x,y, and is {} : {} : {}",x,y,z);
    println!("The value of tup.0, tup.1, tup.2 is {} : {}: {}", tup.0, tup.1, tup.2);

    // Arrays
    let a = [1,2,3,4,5];
    println!("The value of a is {:?}",a);

    let first = a[0];
    let second = a[1];
    println!("The value of first and second is {}:{}", first, second);
    let index=10;
    let element = a[index];

    println!("The value of element is: {}", element);

}
