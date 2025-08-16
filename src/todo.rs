
use std::io;
use std::fmt;
use std::vec;
#[derive(Debug)]
pub struct Todo{
   pub  task: String,
    pub completed:bool,
    
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.task,self.completed)
    }
}
pub struct TodoList {
    pub todos: Vec<Todo>,
}
impl TodoList {
    fn state(&self) {
        
    }
     pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }
   pub  fn add_task(&mut self) {
   
    loop {
        
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
        };

        self.todos.push(todo);

        println!("--- Todo List ---");
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}: {}", i + 1, todo);
        }
    }
}

    }
    pub fn complete(&self){

    }
}