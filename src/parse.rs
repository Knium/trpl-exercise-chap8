use Order;

pub fn parse_order(input: String) -> Order {
    let mut orders = Vec::new();
    for w in input.split_whitespace() {
        orders.push(w);
    }

    match orders.get(0) {
        Some(&"Add") => parse_add(orders),
        Some(&"Show") => parse_show(orders),
        Some(&"Exists") => parse_exists(orders),
        Some(&"Exit") => Order::Exit,
        _ => Order::Error("couldn't parse what you want.".to_string())
    }
}

fn parse_add(orders: Vec<&str>) -> Order {
    if let (Some(name), Some(dpmt)) = (orders.get(1), orders.get(3)) {
        Order::Add(dpmt.to_string(), name.to_string())
    } else {
        Order::Error("ADD".to_string())         
    }
}

fn parse_show(orders: Vec<&str>) -> Order {
  if let Some(dpmt) = orders.get(1) {
      Order::Show(dpmt.to_string())
  } else {
      Order::Error("SHOW".to_string())
  }
}

fn parse_exists(orders: Vec<&str>) -> Order {
    if let (Some(name), Some(&"in"), Some(dpmt)) = (orders.get(1), orders.get(2) ,orders.get(3)) {
        Order::Exists(dpmt.to_string(), name.to_string())
    } else {
        Order::Error("EXISTS".to_string())
    }
}
