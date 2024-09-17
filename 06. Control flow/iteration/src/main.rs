fn main() {
    
    // infinite loop which prints loop
    // loop {
    //     println!("Loop!");
    // }

    let mut num = 0;
    'counter: loop{ //creates a loop with a name
        println!("Count: {}", num);
        let mut decrease = 5;

        loop{
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break; //this breaks the loop
            }
            if num == 2{
                break 'counter; //this will break out of the named loop
            }
            decrease -=1;
        }
        num += 1;

    }

    //while loops

    num = 0;

    while num < 5{
        println!("Num: {}", num);
        num +=1;
    }


    //for loop

    let vec: Vec<i8> = (0..10).collect();

    for element in vec{
        println!("element: {}", element);
    }


    for number in (1..4).rev(){
        println!("{}", number);
    }
    println!("Liftoff!")

}
