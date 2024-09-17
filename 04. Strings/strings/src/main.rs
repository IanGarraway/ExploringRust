fn main() {
    let name = String::from("Ian");

    let course = "Rust".to_string();

    let new_name = name.replace("Ian", "IanG");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "String slice" or "stir"

    let str1 = "hello"; //&str - string slice

    //println!("{}", str1.bogus() );

    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    //compare string == and != (equal and not equal)

    println!("{}", "ONE".to_lowercase()=="one");

    // String literals

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
    

}
