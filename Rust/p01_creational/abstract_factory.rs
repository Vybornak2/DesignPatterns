// Abstract Factory Pattern

// Abstract Products

trait Engine {
    fn get_specs(&self) -> String;
}

trait Transmission {
    fn get_type(&self) -> String;
}

// Concrete Products (Sports Family)

struct HighPerformanceEngine;
impl Engine for HighPerformanceEngine {
    fn get_specs(&self) -> String {
        "V8 Turbo 300hp".to_string()
    }
}

struct ManualTransmission;
impl Transmission for ManualTransmission {
    fn get_type(&self) -> String {
        "6-Speed Manual".to_string()
    }
}

// Concrete Products (Family Family)

struct EfficientEngine;
impl Engine for EfficientEngine {
    fn get_specs(&self) -> String {
        "Inline-4 Diesel 120hp".to_string()
    }
}

struct AutomaticTransmission;
impl Transmission for AutomaticTransmission {
    fn get_type(&self) -> String {
        "CVT Automatic".to_string()
    }
}

// The Abstract Factory

trait CarPartFactory {
    fn create_engine(&self) -> Box<dyn Engine>;
    fn create_transmission(&self) -> Box<dyn Transmission>;
}
// NOTE: Trait Objects
// trait objects size depends on concrete type
// Trait objects are:
// - unsized (size not known at compile time)
// - DST (Dynamically Sized Types)
//
// NOTE: Box<dyn Trait>
// dynamic dispatch for trait objects @ runtime
// Box fixed size pointer to heap-allocated data, allowing us to work with trait objects

// Concrete Factories

struct SportsFactory;
impl CarPartFactory for SportsFactory {
    fn create_engine(&self) -> Box<dyn Engine> {
        Box::new(HighPerformanceEngine)
    }
    fn create_transmission(&self) -> Box<dyn Transmission> {
        Box::new(ManualTransmission)
    }
}

struct FamilyFactory;
impl CarPartFactory for FamilyFactory {
    fn create_engine(&self) -> Box<dyn Engine> {
        Box::new(EfficientEngine)
    }
    fn create_transmission(&self) -> Box<dyn Transmission> {
        Box::new(AutomaticTransmission)
    }
}

// Client Code

struct CarAssembler {
    engine: Box<dyn Engine>,
    transmission: Box<dyn Transmission>,
}

// NOTE: Static Dispatch with Generics
// struct CarAssembler<E: Engine, T: Transmission> {
//     engine: E,
//     transmission: T,
// }
//
// NOTE: Static Dispatch and Polymorphism / Monomorphization
// Monomorphization - Rust generates specific code for each concrete type used with generics at compile time
// in this case comiled code contains two versions of CarAssembler:
// - CarAssembler<HighPerformanceEngine, ManualTransmission>
// - CarAssembler<EfficientEngine, AutomaticTransmission>

impl CarAssembler {
    fn new(factory: &dyn CarPartFactory) -> Self {
        Self {
            engine: factory.create_engine(),
            transmission: factory.create_transmission(),
        }
    }

    fn show_specs(&self) {
        println!("Engine: {}", self.engine.get_specs());
        println!("Transmission: {}", self.transmission.get_type());
    }
}

fn main() {
    let sports_factory: Box<dyn CarPartFactory> = Box::new(SportsFactory);
    let family_factory: Box<dyn CarPartFactory> = Box::new(FamilyFactory);

    let sports_car = CarAssembler::new(&*sports_factory);
    println!("Sports Car Specs:");
    sports_car.show_specs();

    let family_car = CarAssembler::new(&*family_factory);
    println!("\nFamily Car Specs:");
    family_car.show_specs();

    // NOTE: Dereferencing Box<dyn Trait>
    // Box is a smart pointer - access inner value with * operator
    // &*sports_factory - dereference Box to get &dyn CarPartFactory
}
