fn main() {
    let tup = (500, "hi", true);

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
