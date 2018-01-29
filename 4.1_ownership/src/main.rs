
#![feature(core_intrinsics)]

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    //    let s = "hello";
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("Hello s => {}", s);
    print_type_of(&s);

    let bytes = s.as_bytes();
    println!("Bytes of s => {:?}" , bytes);
    print_type_of(&bytes);

    println!("first byte of bytes {}", bytes[0]);
    println!("second byte of bytes {}", bytes[1]);
    println!("length of bytes {}", bytes.len());

    let mut dst = [0,0,0,0,0,0,0,0,0,0,0,0,0];
    dst.copy_from_slice(&bytes);
    dst[0] = 98;
    println!("Bytes of dst => {:?}" , dst);

    let mut dst2 = vec![0; 13];
    dst2.copy_from_slice(&bytes);
    dst2[1] = 111;

    println!("Bytes of dst2 => {:?}" , dst2);

    let mut dst3 = vec![0; bytes.len()];
    dst3.copy_from_slice(&bytes);
    dst3[2] = 222;

    println!("Bytes of dst3 => {:?}" , dst3);

}
