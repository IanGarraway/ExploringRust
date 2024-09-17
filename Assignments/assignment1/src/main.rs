fn main() {
    
    //q1
    let val1 = 5;
    let val2 = 2;
    let ans = val1%val2;

    println!("{}", ans); //1


    //q2
    let mut vec = vec![2,4,6,8,10];
    println!("{:?}", vec); //[2, 4, 6, 8, 10]
    vec.pop();
    vec.push(12);
    println!("{:?}", vec); //[2, 4, 6, 8, 12]

    //q3
    let word = "Hello".to_string();
    let concatWord = concat_string(word);
    println!("conc: {}", concatWord); //conc: HelloWorld

    //q4

    control_flow(1);
    control_flow(2);
    control_flow(28);
    control_flow(51);


}

//q3

fn concat_string(new_word:String)->String{
    let concatWord = new_word + "World";
    concatWord
}


//q4

fn control_flow(num: u64){
    if num == 1{
        println!("The value is one");
    } else if num > 50{
        println!("The value is greater than 50");
    } else if num > 25{
        println!("The value is greater than 25 but less than 50");
    } else {
        println!("The value is less than 25");
    }
}