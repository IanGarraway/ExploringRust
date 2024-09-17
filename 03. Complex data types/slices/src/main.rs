fn main() {
    let mut v: Vec<i32>=(0..5).collect();

    println!("{:?}", v); //[0, 1, 2, 3, 4];

    let sv: &[i32] = &v[2..4]; //this is a 'fat' pointer, a reference. non owning
    println!("{:?}", sv);   //[2, 3]
    
} 
