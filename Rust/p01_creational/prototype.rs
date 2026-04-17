// Prototype pattern

use std::rc::Rc;

#[derive(Clone, Debug)]
struct Circle {
    pub radius: u32,
    pub color: String,
    tracker: Rc<()>, 
}

fn print_count(circle: &Circle) {
    println!("Current count: {}", Rc::strong_count(&circle.tracker));
}

fn main() {
    let circle1 = Circle {
        radius: 15,
        color: String::from("red"),
        tracker: Rc::new(()), // Initialize with a unit value
    };

    print_count(&circle1); // Count: 1

    let mut circle2 = circle1.clone();
    circle2.radius = 20;
    circle2.color = String::from("blue");
    
    println!("Circle 1: {:?}", circle1);
    println!("Circle 2: {:?}", circle2);

    print_count(&circle1); // Count: 2

    {
        let _circle3 = circle1.clone();
        print_count(&circle1); // Count: 3
    } // _circle3 is dropped here

    print_count(&circle1); // Count: 2 (since _circle3 is dropped)
}
