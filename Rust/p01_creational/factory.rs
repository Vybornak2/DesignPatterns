// Factory Method Pattern

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

// NOTE: Error Handling Options
// - Result<T, E>: Used for recoverable errors
// - Option<T>: Used when a value may be absent (None) or present (Some)
// - panic!: Used for unrecoverable errors, will terminate the program
// - Error Propagation: Use the ? operator to propagate errors up the call stack
// - unwrap, unwrap_or, unwrap_or_else: Methods to handle Result and Option values
// - expect: Similar to unwrap but allows you to provide a custom error message

// NOTE: Result Matching vs Unwrap / Expect
// - Matching: Allows you to handle both success (Ok) and error (Err) cases
// - Unwrap: Panics if the Result is an Err, should be used when you are sure it won't fail or in tests
// - Expect: Similar to unwrap but allows you to provide a custom error message, useful for debugging

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
