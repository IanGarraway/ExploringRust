use rand::seq::SliceRandom; //RNG utility. Needs version adding to the cargo.toml
use rand::thread_rng;

fn main() {
    let mut nums: Vec<i32> = vec![]; //vectors seem to be Rust's Stack collection

    nums.push(1); //push, add to the top of the vector
    nums.push(2);
    nums.push(3);

    let pop = nums.pop(); //take the top of the vector off.
    //Option<T>, return none or Some(T)

    println!("{:?}", pop);

    let two = nums[1]; //copy
    //&nums[1], creates a reference if copy is not available

    println!("{}", two);

    let one = nums.first(); //return an Option<T>, so None if vec is empty, or Some<T> is [0]
    println!("{:?}", one);

    // .last
    // .first_mut and .last_mut, so will borrow mutable references

    println!("{}", nums.len()); //length value
    println!("{}", nums.is_empty()); //bool

    //some of the vector options
    nums.insert(0, 10);
    nums.insert(3,12);
    nums.insert(2,25);

    nums.remove(3);

    nums.sort();
    println!("{:?}", nums);

    nums.reverse();
    println!("{:?}", nums);

    nums.shuffle(&mut thread_rng()); //randomises vector based on the supplied seed
    println!("{:?}", nums);

    

}
