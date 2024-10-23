mod list {
    pub struct Tasks{
        pub item: String,
    }

        
}



mod things_todo; //looks for rs file with the same name
use crate::things_todo::add_activity; //looks for a function in the rs file
use things_todo::items_completed; //use command allows you to simplify the path so you can just use the 
use things_todo::items_completed::test::test; //name of the function or the module

fn lets_add_task(){
    let task = list::Tasks {item: String::from("Tasks")};
    
    add_activity();
    //things_todo::add_activity(); //relative path
    //crate::things_todo::add_activity(); //absolute path because we start at the root crate

    items_completed::remove_task();
    test();

}