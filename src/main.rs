mod parse;
mod order;

use parse::parse_order;
use order::Order;
use std::collections::HashMap;

fn read_stdin() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to input stdin");
    s
}

fn main() {
    let mut map = HashMap::new();
    loop {
        let s = read_stdin();
        let orders = parse_order(s);
        match orders {
            Order::Exit => break,
            Order::Error(s) => {
                println!("Parse error happend: {}", s);
                break
            },
            _ => orders.execute(&mut map)
        }
    }
}
