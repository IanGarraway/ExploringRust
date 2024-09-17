fn main() {
    //i8 = 8bit signed integer
    let x: i8 = 10;
    println!("{}", x);

    // u8 = 8bit unsigned int
    let y: u8 = 10;
    println!("{}",y);

    // the following are all 255
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;


    println!("{}",decimal);
    println!("{}",hex);
    println!("{}",octal);
    println!("{}",binary);

    // b is byte value of the letter (A = 65).
    let byte = b'A';
    println!("{}", byte);

    //floating points
    let float = 2.0; //f64 default

    let float32: f32 = 1.0;


    //booleans - can be implictly or explicitly stated.
    let t = true;
    let f: bool = false;

    let c = 'c';

    println!("{}", c);

    // arithmatic

    // + - * / %

    let a = 10;
    let b = 4;

    let remainder = a%b;
    println!("{}", remainder);
}
