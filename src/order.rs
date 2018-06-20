use std::collections::HashMap;

#[derive(Debug)]
pub enum Order {
    Add(String, String),
    Show(String),
    Exists(String, String),
    Error(String),
    Exit
}

impl Order {
    pub fn execute(&self, map:&mut HashMap<String, Vec<String>>) {
        match self {
            Order::Add(dpmt, name) => add(dpmt.to_string(), name.to_string(), map),
            Order::Show(dpmt) => show(dpmt.to_string(), map),
            Order::Exists(dpmt, name) =>  println!("{}", exists(dpmt.to_string(), name.to_string(), map)),
            _ => println!("Order::UnImplemented")
        }
    }
}

fn add(dpmt: String, name: String, map: &mut HashMap<String, Vec<String>>) {
    let names = map.entry(dpmt).or_insert(vec![]);
    names.push(name);
    names.sort();
    println!("done");
}

fn show(dpmt: String, map: &HashMap<String, Vec<String>>) {
    println!("{} contains below members", dpmt);
    if let Some(members) = map.get(&dpmt) {
        for member in members {
            println!("  - {}", member);
        }
    }
}

fn exists(dpmt: String, name: String, map: &HashMap<String, Vec<String>>) -> bool {
    if let Some(members) = map.get(&dpmt) {
        for member in members {
            if &name == member {
                return true;
            }
        }
    }
    false
}
