use crate::todo::Todo;
use std::fmt;
use crate::todo::TodoList;

mod todo;

fn main() {
    let mut instance = TodoList::new();

    instance.add_task();
    instance.add_task();

    instance.complete();
     println!("\nAfter completing first task:");
    for (i, todo) in instance.todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo);
    }
}