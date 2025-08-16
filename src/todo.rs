use std::io;
use std::fmt;
use std::sync::BarrierWaitResult;
use std::vec;
#[derive(Debug)]
pub struct Todo{
   pub  task: String,
    pub completed:bool,
    
}//creating a struct todo to store the tasks

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.task,self.completed)
    }
}//so that we can normally println todo
pub struct TodoList {
    pub todos: Vec<Todo>,//stodring multiple todos.
}//creating a struct to store a vec to store multiple todos
impl TodoList {//implementation for struct todolist
    pub fn state(&mut self) {
       loop {
            println!("Welcome to the Todo CLI app!");
        println!("type /help to get started");
        let mut a = String::new();
        io::stdin().read_line(&mut a ).expect("Enter a valid command");
        match a.trim() {
            "/help"=>self.help(),
            "add"=>self.add_task(),
            "rem"=>self.remove_task(),
            
            _ =>println!("Invalid Command"),
        }
        if a.trim()=="q" {
  break;
        }
       }
        //empty as of now. will ocntain normal state of the app
    }
    pub fn help(&self) {
        println!("Here is the list of available commands: \n1.add: to add new tasks\n2.rem:to remove existing tasks\n3.comp:Mark tasks as completed\n4.view:View existing tasks\n5./help:see list of available commands");
    }
     pub fn new() -> Self {
        TodoList { todos: Vec::new() }//create a new vec and start the aopp
    }
   pub  fn add_task(&mut self) {
   
    loop {
        //loop till someone presses q
         let mut a = String::new();

        println!("Enter a task:");
        io::stdin().read_line(&mut a).expect("Enter a valid task.");
         if a.trim()=="q" {
             break;
         }
        else {
            
        let todo = Todo {
            task: a,
            completed: false,
        };//create instance

        self.todos.push(todo);//push to vec

        println!("--- Todo List ---");
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}: {}", i + 1, todo);
        }//print
    }
    }
   }
    pub fn remove_task(&mut self){
        loop{
            let mut b = String::new();
            println!("Enter the task you want to remove");
            io::stdin().read_line(&mut b).expect("Enter a valid task");
            if b.trim() == "q"{
                break;
        }
        else{
            self.todos.retain(|x| x.task.trim() != b.trim());
            println!("Updated Todo List:");
            for (i, todo) in self.todos.iter().enumerate() {
                println!("{}: {}", i + 1, todo);
            }
        }
    }
    }

    
    pub fn complete(&self){
// update bool value to complete tasks. not remove them entirely.
    }
}
