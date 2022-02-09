use std::collections::HashMap;
use std::io::Error;
use std::io::Read;
use std::str::FromStr;

const DB_FILE: &str = "db.txt";

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}


impl Todo {

    fn new() -> Result<Todo, Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(DB_FILE)?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, "\t").collect::<Vec<&str>>())
            .map(|ary| (ary[0], ary[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn set(&mut self, key: String, value: bool) {
        match self.map.get_mut(&key) {
          Some(v) => Some(*v = value),
          None => None,
        };
    }

    fn complete(&mut self, key: String) {
        self.set(key, true);
    }

    fn incomplete(&mut self, key: String) {
        self.set(key, false);
    }

    fn delete(&mut self, key: String) {
        self.map.remove(&key);
    }

    fn pretty_list(&mut self) -> String {
        let mut list = String::new();
        for (task, done) in &self.map {
            let checkmark = if *done { "✓" } else { " " };
            let record = format!("[{}] {}\n", checkmark, task);
            list.push_str(&record);
        }
        list
    }

    fn save(self) -> Result<(), Error> {
        // take ownership of self. this should be the last
        // method called for Todo
        
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        std::fs::write(DB_FILE, content)
    }

}

#[derive(Debug)]
enum Action {
    Add, 
    Done,
    Undone,
    Delete,
    List,
}

fn main() {
    //println!("Hello, world!");
    let params: Vec<String> = std::env::args().collect();
    let command = &params[1];
    let mut argument: String = params[2..].join(" "); 

    //let action = std::env::args().nth(1).expect("Please specify an action");
            //.map(|p| println!("{}", p));
    //println!("{:?}", &params[1..]);

    

    //let actions = Actions {
         //add: String::from("add")
    //};

    let mut todo = Todo::new().expect("Initialisation of db failed");

    let action = match command.as_str() {
        "a" | "add" | "insert" | "touch" => Action::Add,
        "r" | "rem" | "rm" | "del" | "delete" | "destroy" => Action::Delete,
        "d" | "do" | "done" | "complete"  => Action::Done,
        "u" | "undo" | "undone"  => Action::Undone,
        "v" | "view" | "show" | "ls" | "list" => Action::List,
        _ => {
            argument = format!("{} {}", command, argument);
            Action::Add
        },
    };


    // do whatever in the todolist
    match action {
        Action::Add => todo.insert(argument),
        Action::Done => todo.complete(argument),
        Action::Undone => todo.incomplete(argument),
        Action::Delete => todo.delete(argument),
        Action::List => print!("{}", todo.pretty_list()),
    }

    //println!("{:#?}", action);

    match todo.save() {
        Ok(_) => println!("-- rastadone v 420--"), 
        Err(why) => println!("bruh {}", why), 
    }
}

