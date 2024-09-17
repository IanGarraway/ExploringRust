fn main() {

    //Vectors are a resizable array of elements allocated on the heap
    // dynamic Lists?

    //creating a vector with data in it
    let mut nums = vec![1,2,3];

    nums.push(4);

    println!("{:?}", nums);  //[1, 2, 3, 4]
    nums.pop();
    println!("{:?}", nums);  //[1, 2, 3]


    //creating an empty vector
    let mut vec = Vec::new(); //vec!
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);  //["Test", "String"]

    vec.reverse();
    println!("{:?}", vec);  //["String", "Test"]


    //creating an empty vector of a predetermined length
    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());  //2


    //creating a vector with a iterator

    let v: Vec<i32> = (0..5).collect(); //create 0 to 5, inclusive of 0, exclusive of 5. 
    println!("{:?}", v); //[0, 1, 2, 3, 4]


}
