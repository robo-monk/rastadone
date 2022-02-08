use std::collections::HashMap;
use std::io::Error;

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
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

        std::fs::write("db.txt", content)
    }
}

fn main() {
    //println!("Hello, world!");
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);

    let mut todo = Todo {
        map: HashMap::new(),
    };
    
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"), 
            Err(why) => println!("bruh {}", why), 
        }
    }
}
