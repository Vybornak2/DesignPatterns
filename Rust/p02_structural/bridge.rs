// Bridge pattern

// NOTE: Implementor
// defines the interface for implementation classes

trait Implementor {
    fn operation_impl(&self) -> String;
}

struct ConcreteImplementorA;

impl Implementor for ConcreteImplementorA {
    fn operation_impl(&self) -> String {
        "ConcreteImplementorA".to_string()
    }
}

struct ConcreteImplementorB;

impl Implementor for ConcreteImplementorB {
    fn operation_impl(&self) -> String {
        "ConcreteImplementorB".to_string()
    }
}

// NOTE: Abstraction
// defines the interface for a client
// maintains a reference to an object of type Implementor

struct Abstraction<T: Implementor> {
    implementor: T,
}

impl<T: Implementor> Abstraction<T> {
    fn new(implementor: T) -> Self {
        Abstraction { implementor }
    }

    fn operation(&self) -> String {
        self.implementor.operation_impl()
    }
}

// NOTE: Generics - impl<T: Implementor> Abstraction<T>
// defines a type bound by the Implementor trait

fn main() {
    let implementor_a = ConcreteImplementorA;
    let abstraction_a = Abstraction::new(implementor_a);
    println!("Abstraction with ConcreteImplementorA: {}", abstraction_a.operation());

    let implementor_b = ConcreteImplementorB;
    let abstraction_b = Abstraction::new(implementor_b);
    println!("Abstraction with ConcreteImplementorB: {}", abstraction_b.operation());
}
