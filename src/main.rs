use crate::todo::Todo;
use std::fmt;
use crate::todo::TodoList;

mod todo;

fn main() {
    let mut instance = TodoList::new(); //creating an instance of the TodoList Struct to  use it in main.rs

    instance.add_task();//calling add_task
    

    instance.complete();
     println!("\nAfter completing first task:");
    for (i, todo) in instance.todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo);//this entire thing is to print the tasks.
    }
}