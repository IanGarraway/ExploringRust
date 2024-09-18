fn main() {

    //q1:
    //Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7. 
    //Inside of this function check the first value of the vector and see if it is equal to one. If the value is equal 
    // to one, then return true, otherwise return false. Next add the value 15 to the vector. Print out the vector to confirm 
    // your results.

    let mut val = vec![1,3,5,7];

    let is_one = check_first(val[0]);

    println!("first element is one? {}", is_one);
    val.push(15);
    println!("{:?}", val);

    //q2
    // Create a function called "add_two". This function is going to take one parameter that is i8 and add two to it.
    // For the function, do you have to pass the value by reference in order for you to maintain ownership of it inside of main?

    let val = add_two(3);
    println!("3+2 = {}", val);

}

//q1 function
fn check_first(val: i32)->bool{
    if val==1{
        true
    } else{
        false
    }
}

//q2 function
fn add_two(mut val: i8)->i8{
    val= val+2;
    val
}
