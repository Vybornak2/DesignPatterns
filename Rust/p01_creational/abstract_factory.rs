// Abstract Factory Pattern
// --- 1. Abstract Products (Traits) ---

trait Engine {
    fn get_specs(&self) -> String;
}

trait Transmission {
    fn get_type(&self) -> String;
}

// --- 2. Concrete Products (Sports Family) ---

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

// --- 3. Concrete Products (Family Family) ---

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

// --- 4. The Abstract Factory ---

trait CarPartFactory {
    fn create_engine(&self) -> Box<dyn Engine>;
    fn create_transmission(&self) -> Box<dyn Transmission>;
}

// --- 5. Concrete Factories ---

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

// --- 6. The Client ---

struct CarAssembler {
    engine: Box<dyn Engine>,
    transmission: Box<dyn Transmission>,
}

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
    // Choose the factory (theme)
    let sports_factory: Box<dyn CarPartFactory> = Box::new(SportsFactory);
    let family_factory: Box<dyn CarPartFactory> = Box::new(FamilyFactory);

    let sports_car = CarAssembler::new(&*sports_factory);
    println!("Sports Car Specs:");
    sports_car.show_specs();

    let family_car = CarAssembler::new(&*family_factory);
    println!("\nFamily Car Specs:");
    family_car.show_specs();
}
