// Factory Method Pattern
//

pub trait Transport {
    fn deliver(&self);
}

struct Truck;

impl Transport for Truck {
    fn deliver(&self) {
        println!("Delivering by truck");
    }
}

struct Ship;

impl Transport for Ship {
    fn deliver(&self) {
        println!("Delivering by ship");
    }
}

// enum or Result should be implemented
fn get_transport(name: &str) -> Result<Box<dyn Transport>, String> {
    match name {
        "truck" => Ok(Box::new(Truck)),
        "ship" => Ok(Box::new(Ship)),
        other => Err(format!("Unknown transport type: {}", other)),
    }
}

fn main() {
    let transport = get_transport("truck");
    match transport {
        Ok(t) => t.deliver(),
        Err(e) => println!("Error: {}", e),
    }

    let transport = get_transport("ship");
    match transport {
        Ok(t) => t.deliver(),
        Err(e) => println!("Error: {}", e),
    }

    let transport = get_transport("plane");
    match transport {
        Ok(t) => t.deliver(),
        Err(e) => println!("Error: {}", e),
    }
}
