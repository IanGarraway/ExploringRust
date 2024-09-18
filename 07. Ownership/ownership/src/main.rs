fn main() {
    // let var = 1; //created on stack
    // let mut s = "hello".to_string(); //created on the heap

    // s.push_str(",world");

    // let x = vec!["tyler".to_string()];
    // let y = x; //this moves ownership of the data in x to y, not make
    // //a copy of it.

    // println!("{:?}",x); //x would give an error, as it no longer owns the data

    
    // let x = vec!["tyler".to_string()];
    // let y = x.clone(); //this makes a deep copy of the data in x to y, 

    // println!("{:?}",x); //x would no longer give an error, as it still owns the data

    // let x = 1; //in this example y gets a copy of x, not the data. 
    // let y = x; //integrals I think...

    // println!("x = {}, y = {}", x, y);

    let s = String::from("takes");
    takes_ownership(s);
    // println!("{}", s);

    let val = 1;
    make_copy(val);

    let str1: String = give_ownership();
    println!("{}", str1);

    let str3 : String = take_and_give(str1);

    println!("{}",str3);


    looping_example();
    references();

}

fn takes_ownership(s: String){
    let strin = s;
    println!("{}", strin)
}

fn make_copy(one: i32){
    let val1 = one;
    println!("{}", val1)
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String{
    str2
}

// var is dropped, s is dropped


fn looping_example(){

    // let mut str1 = String::from("Tyler");
    // let mut str2: String;

    // loop{
    //     str2 = str1; //errors because str1 doesn't have ownership after the first iteration
    // }
}

fn references(){

    let mut s = String::from("hello");
    change_string(&mut s); //the & is used to make a reference
    println!("{}", s);


}

fn change_string(some_string: &mut String){
    some_string.push_str(", world!");
}
