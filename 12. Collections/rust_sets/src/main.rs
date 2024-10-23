use std::collections::HashSet;
fn main() {

    let mut hs = HashSet::new();

    //len()
    // is_empty()

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);
    hs.insert(5);

    for x in hs.iter(){
        println!("Iter: {}", x);
    }

    // hs.remove(&2);

    // for x in hs.iter(){
    //     println!("Iter: {}", x);
    // }
    

    let mut hs2 = HashSet::new();

    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);
    hs2.insert(11);

    for x in hs.intersection(&hs2){
        println!("Intersection: {}", x);
    }

    let intersection = &hs & &hs2; //shorthand intersection using the binary bitwise & operator

    for x in intersection{
        println!("Intersection: {}", x);
    }

    let union = &hs| &hs2; //short hand for .union command

    for x in union{
        println!("Union: {}", x);
    }

    let dif = &hs - &hs2; //shorthand for hs.difference(other), things in hs, but not in hs2

    for x in dif{
        println!("Difference: {}", x)
    }

}
