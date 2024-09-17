fn main() {
    let array = [1,2,3];

    println!("{}", array[0]);


    //Arrays by default, aren't mutable, make sure to declare them mut!
    let mut array2: [i32; 3]= [4,5,6];
    println!("{}", array2[0]);

    array2[0] = 10;
    println!("{}", array2[0]);

    //Compiler will warn if attempting to out of bounds...

    //Panic??!
    
    //println!("{}", array2[3]);


}
