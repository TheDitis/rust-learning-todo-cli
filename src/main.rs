use std::env::args;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let action = args().nth(1).expect("An argument is");
    let item = args().nth(2);

    let mut todo = Todo::new().expect("Initialization of data file failed");

    match &action.to_lowercase()[..] {
        "-n" | "-a" | "add" => {
            match item {
                Some(item) => {
                    todo.add(item);
                    match &todo.save() {
                        Ok(_) => println!("todo saved!"),
                        Err(e) => println!("There was an error: {:?}:", e),
                    }
                },
                None => println!("Error: A second argument is required to add")
            }
        },
        "-C" | "clear" => {
            let force = match item {
                Some(arg) => match &arg.to_lowercase()[..] {
                    "-f" | "--force" => true,
                    _ => false,
                },
                None => false,
            };
            match &todo.clear(force) {
                Ok(message) => println!("{:?}", message),
                Err(err) => println!("ERROR: {:?}", err),
            }
        },
        "r" | "remove" | "delete" => {
            match item {
                Some(item) => {
                    todo.remove(item).unwrap();
                },
                None => {
                    println!("remove/delete requires a second argument.");
                },
            };
        },
        "-c" | "check" | "finish" => {
            match item {
                Some(item) => {
                    todo.update_item(&item, true);
                },
                None => {
                    println!("check/finish requires a second argument.")
                }
            }
        },
        "-u" | "uncheck" | "undo" => {
            match item {
                Some(item) => {
                    todo.update_item(&item, false);
                },
                None => {
                    println!("uncheck/undo requires a second argument.");
                }
            }
        },
        "-p" | "print" => {
            todo.print();
        },
        "done" => {
            todo.print_done();
        },
        "todo" => {
            todo.print_not_done();
        },
        _ => println!("{:?} is not a valid action.", action)
    };
}


/** MAIN STRUCT */
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("data.txt")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
    fn _print(&self, map: Vec<(&String, &bool)>) {
        println!("{{");
        for (key, val) in map {
            println!("\t{}: {}", key, val)
        }
        println!("}}");
    }
    fn print(&self) {
        self._print(self.map.iter().collect())
    }
    fn print_done(&self) {
        let map = self.map.iter().filter(|&(_, v)| *v).collect();
        self._print(map);
    }
    fn print_not_done(&self) {
        let map = self.map.iter().filter(|&(_, v)| !*v).collect();
        self._print(map)
    }
    fn add(&mut self, key: String) {
        if !self.map.contains_key(&key) {
            self.map.insert(key, false);
        } else {
            println!("item already exists, skipping");
        }
    }
    fn save(&self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in &self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("data.txt", content)
    }
    fn remove(&mut self, item: String) -> Result<&str, std::io::Error> {
        if self.map.contains_key(&item) {
            let res = match self.map.remove(&item) {
                Some(v) => v,
                None => false,
            };
            if res {
                Ok("Item removed successfully")
            } else {
                Ok("There was a problem removing the item")
            }
        } else {
            Ok("That item doesn't exist")
        }
    }
    fn clear(&self, force: bool) -> Result<&str, &str> {
        if force {
            self._delete_data_file()
        } else {
            let response = get_input(
                "YOU ARE ABOUT TO CLEAR ALL OF YOUR TODOS! ARE YOU SURE YOU WANT TO? (yes/no): "
            );

            match &response.to_lowercase()[..] {
                "y" | "yes" => self._delete_data_file(),
                "n" | "no" => Err("Aborting clear operation"),
                _ => Err("Invalid Input"),
            }
        }
    }
    fn update_item(&mut self, item: &str, is_done: bool) {
        let modifier = if is_done { String::from("") } else { String::from("not ") };
        if self.map.contains_key(item) {
            if self.map[item] == is_done {
                println!("item was already marked as {}done", modifier);
            } else {
                self.map.insert(item.to_string(), is_done);
                match self.save() {
                    Ok(_) => println!("item marked as {}done!", modifier),
                    Err(e) => println!("There was an error: {:?}:", e),
                }
            }
        } else {
            println!("item doesn't exist!")
        }
    }

    fn _delete_data_file(&self) -> Result<&str, &str> {
        match std::fs::remove_file("data.txt") {
            Ok(_) => Ok("All notes cleared!"),
            Err(_) => Err("There was an error deleting your notes ):"),
        }
    }
}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}