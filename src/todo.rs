
use std::io;
use std::fmt;
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
    fn state(&self) {
        //empty as of now. will ocntain normal state of the app
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
         if a=="q" {
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
    pub fn complete(&self){
// update bool value to complete tasks. not remove them entirely.
    }
}