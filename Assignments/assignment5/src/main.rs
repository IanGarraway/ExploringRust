// Complete the following questions.
// Questions for this assignment

//1 Modify the solution to the Section 4 assignment by creating a Trait that has the set_mpg, set_color, and set_top_speed methods. 
//Then create a Motorcycle struct with the same fields as the Car struct: mpg, color, and top_speed. Now implement your Trait on both the Car and 
//Motorcycle struct. Print out the results to confirm a working solution.

//2 Create a simple print function that uses Generic T. This Generic T will need to implement std::fmt::Debug depending on the values you pass in. 
//Our function takes one parameter of type T. Our function will then print out the value that is passed in.
struct Car{
    mpg: u32,
    color: String,
    top_speed: u32,
}

struct Motorcycle{
    mpg: u32,
    color: String,
    top_speed: u32,
}

trait Vehicle{
    fn set_mpg(&mut self, new_mpg: u32);    
    fn set_color(&mut self, new_color: String);
    fn set_top_speed(&mut self, new_top_speed: u32);
    fn get_mpg(&self) -> u32;
    fn get_color(&self) -> &str;
    fn get_top_speed(&self) -> u32;

    fn overview(&self) -> String {
        format!(
            "Color: {}, MPG: {}, Top Speed: {}",
            self.get_color(),
            self.get_mpg(),
            self.get_top_speed()
        )
    }
}

impl Vehicle for Car {
    fn set_mpg(&mut self, new_mpg: u32){
        self.mpg=new_mpg;
    }

    fn set_color(&mut self, new_color: String){
        self.color=new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: u32){
        self.top_speed = new_top_speed;
    }
    fn get_mpg(&self) -> u32 {
        self.mpg
    }

    fn get_color(&self) -> &str {
        &self.color
    }

    fn get_top_speed(&self) -> u32 {
        self.top_speed
    }
    
}

impl Vehicle for Motorcycle {
    fn set_mpg(&mut self, new_mpg: u32){
        self.mpg=new_mpg;
    }

    fn set_color(&mut self, new_color: String){
        self.color=new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: u32){
        self.top_speed = new_top_speed;
    }
    fn get_mpg(&self) -> u32 {
        self.mpg
    }

    fn get_color(&self) -> &str {
        &self.color
    }

    fn get_top_speed(&self) -> u32 {
        self.top_speed
    }
    
}





fn main() {
    let mut car1 = Car{mpg:5, color:String::from("Blue"), top_speed:200 };
    println!("{} {} {}", car1.color, car1.mpg, car1.top_speed);
    car1.set_color(String::from("Red"));
    car1.set_mpg(10);
    car1.set_top_speed(250);
    println!("{} {} {}", car1.color, car1.mpg, car1.top_speed);

     let mut bike1 = Motorcycle{mpg:15, color:String::from("Black"), top_speed:250 };
    println!("{} {} {}", bike1.color, bike1.mpg, bike1.top_speed);
    bike1.set_color(String::from("Red"));
    bike1.set_mpg(10);
    bike1.set_top_speed(280);
    println!("{} {} {}", bike1.color, bike1.mpg, bike1.top_speed);

    printer(&bike1);

    

}

fn printer<T: Vehicle>(item: &T){
    println!("Overview: {}", item.overview())
}