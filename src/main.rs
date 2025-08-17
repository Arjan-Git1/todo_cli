use crate::todo::Todo;
use std::fmt;
use crate::todo::TodoList;

mod todo;

fn main() {
    let mut instance = TodoList::new(); //creating an instance of the TodoList Struct to  use it in main.rs

    instance.state();
}