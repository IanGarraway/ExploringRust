use std::string;

//Generics
struct Point<T, Z>{ //Generics, any label using pascal case (cap start) 
    x: T, //Actual data structure is worked out by the data entered
    y: Z, //so the same structure can change data types.
}

//Traits

trait Overview {
    fn overview(&self)-> String {
        String::from("This is a Rust Course!") //this is a default implementation
    }
}

struct Course {
    headline: String,
    author: String,
}

struct AnotherCourse {
    headline: String,
    author: String,
}
struct AlsoACourse {
    headline: String,
    author: String,
}

impl Overview for Course{ //this overrides a default implementation
    fn overview(&self)-> String {
        format!("{}, {}", self.author, self.headline )
    }
}
impl Overview for AnotherCourse{
    fn overview(&self)-> String {
        format!("{}, {}", self.author, self.headline )
    }
}

impl Overview for AlsoACourse { //uses the default implementation
    
}

//drop functions

impl Drop for Course{
    fn drop(&mut self){
        println!("Dropping: {}", self.author)
    }
}

//clone
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self){
        *self = source.clone()
    }
}

//operator overloading

use std::ops::Add;


#[derive(Debug)]
struct Pointa<T> {
    x: T,
    y: T,
}

impl <T> Add for Pointa<T>
    where 
    T: Add<Output = T>{
        type Output = Self;
        fn add(self, _rhs: Self)-> Self{
            Pointa { 
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
            }
        }
    }


fn main() {

    //Generics
    let mut coord = Point{x: 5.0, y: 5.0};
    let coord2 = Point{x: 'a', y: 'b'};
    let coord3 = Point{x:'a', y: 5};

    // coord.x = 'a'; doesn't work data type can't be changed post declaration
    coord.x = 6.0; //does work, data in the structure can change. 

    //Traits
println!("Traits");
    let course1 = Course{headline: String::from("Headline!"), author: String::from("Ian"),};
    let course2 = AnotherCourse{headline: String::from("Different Headline!"), author: String::from("Still Ian"),};
    let course3 = AlsoACourse{headline: String::from("Different Headline!"), author: String::from("Still Ian"),};

    println!("{}", course1.overview());
    println!("{}", course2.overview());
    println!("{}", course3.overview());

    //traits as params

    println!("Traits as Parameters:");

    call_overview(&course1);
    call_overview(&course2);
    call_overview(&course3);

    //Utility Traits
    //Drops

    drop(course1);

    //Operator Overloading
    let coord = Pointa{x: 5.0, y: 5.0};
    let coord2 = Pointa{x: 1.0, y: 2.0};

    let sum = coord+coord2;
    println!("Overloading: {:?}", sum);

}

//traits as parameters

// fn call_overview(item: &impl Overview){
//     println!("Overview: {}", item.overview())
// }
fn call_overview<T: Overview>(item: &T){ //same but using generics
    println!("Overview: {}", item.overview())
}

//fn overview (item1: &impl Overview, item2: @impl Overview)
//fn overview<T: Overview>(item1: &T, item2: &T)
//fn overview(item1: &impl Overview + AnotherTrait)
//fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)

//Utility Traits

