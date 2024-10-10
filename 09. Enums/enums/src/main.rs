enum Pet {dog, cat,fish}

impl Pet{
    fn what_am_i(self) -> &'static str{
        match self{
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
        }
    }
}

enum IpAddrKind{
    V4,
    V6,
}

//enums can also have the data type declared explicitly

enum IpAddrKind2{
    V4(String),
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,

}

fn main() {
    let _dog = Pet::dog;

    println!("{}", _dog.what_am_i());

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    //option
    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i8> = None; //Option<T>, let X = 5 == i32

    //Match

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);


    what_pet("Dog");
    what_pet("Cat");
    what_pet("Cow"); //match is exhaustive, all cases need to be covered.


    //if let

    let dog2 = Some(Pet::dog);
    if let Some(Pet::dog) = dog2{
       println!("The animal is a dog!") 
    } else {
        println!("Not a dog!");
    }
    let dog2 = Some(Pet::cat);
    if let Some(Pet::dog) = dog2{
       println!("The animal is a dog!") 
    } else {
        println!("Not a dog!");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    let x = 1;

    match x{
        1|2 => println!("One or Two"),
        _ => println!("Not one or two"),
    }
    let x = 10;

    match x{
        1|2 => println!("One or Two"),
        _ => println!("Not one or two"),
    }

    match x{
        1..=5 => println!("Matches"),
        _ => println!("Not matching"),
    }

    let x = 5;

    match x{
        1..=5 => println!("Matches"),
        _ => println!("Not matching"),
    }



}


//match
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}

fn what_pet(input: &str){
    match input{
        "Dog" => println!("I have a dog!"),
        "Fish" => println!("I have a Fish!"),
        "Cat" => println!("I have a Cat!"),
        _ => println!("I have no clue what pet you have"), // the _ indicates anything else, match is exhaustive, all possibilities must be covered
    }
}