use std::collections::HashMap;


fn main() {
    
    let mut hm = HashMap::new();

    hm.insert(1, 1);
    hm.insert(5, 2);
    hm.insert(30, 3);
    let old = hm.insert(30, 4); //key is going to update the old value of 3 to 4, also returns old value

    println!("{:?}", hm);
    println!("{:?}", old);

    println!("{}", hm.contains_key(&5));
    println!("{}", hm.contains_key(&8));
    println!("{:?}", hm.get(&5));

    let one = hm.remove(&1); //removes value at key, and returns that value
    println!("{:?}", hm);
    println!("{:?}", one);

    let remove = hm.remove_entry(&5);
    println!("{:?}", remove);

    hm.clear();
    println!("{}", hm.is_empty());

}
