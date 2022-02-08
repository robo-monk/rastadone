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

        let mut map = HashMap::new();

        for entries in content.lines() {
            let mut values = entries.split("\t");
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }

        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
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

fn main() {
    //println!("Hello, world!");
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);

    //let mut todo = Todo {
        //map: HashMap::new(),
    //};
    //
    let mut todo = Todo::new().expect("Initialisation of db failed");

    
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"), 
            Err(why) => println!("bruh {}", why), 
        }
    }
}

