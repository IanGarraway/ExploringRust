// fn main() {
//     // let r;

//     // {
//     //     let x = 5;
//     //     r = &x;  //error 'borrowed value does not live long enough' 
//     // } //x is dropped

//     // println!("{}", r);
// }


// fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str{
//     y
// }

use std::arch::x86_64;

struct MyString<'a>{
    text: &'a str,
}

fn main(){
    let str1 = String::from("This is my string");
    let x = MyString{text: str1.as_str()};
    let s: &'static str = "I have a static lifetime";

}