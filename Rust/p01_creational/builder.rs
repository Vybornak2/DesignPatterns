// Builder Pattern

#[derive(Debug)]
enum Engine {
    Gasoline,
    Diesel,
}

#[derive(Debug)]
enum Gps {
    Basic,
    Advanced,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Car {
    engine: Engine,
    gps: Option<Gps>,
    seats: u16,
}

impl Car {
    fn new(engine: Engine, gps: Option<Gps>, seats: u16) -> Self {
        Car { engine, gps, seats }
    }

    // This is the common "entry point" for a builder
    fn builder() -> CarBuilder {
        CarBuilder::default()
    }
}

trait Builder {
    type Product;
    fn build(self) -> Self::Product;
    fn engine(self, engine: Engine) -> Self;
    fn gps(self, gps: Gps) -> Self;
    fn seats(self, seats: u16) -> Self;
}

#[derive(Debug)]
struct CarBuilder {
    engine: Engine,
    gps: Option<Gps>,
    seats: u16,
}

impl Default for CarBuilder {
    fn default() -> Self {
        CarBuilder {
            engine: Engine::Gasoline,
            gps: None,
            seats: 4,
        }
    }
}

impl Builder for CarBuilder {
    type Product = Car;

    fn build(self) -> Car {
        Car::new(self.engine, self.gps, self.seats)
    }

    fn engine(mut self, engine: Engine) -> Self {
        self.engine = engine;
        self
    }

    fn gps(mut self, gps: Gps) -> Self {
        self.gps = Some(gps);
        self
    }

    fn seats(mut self, seats: u16) -> Self {
        self.seats = seats;
        self
    }
}

struct Director;

impl Director {
    fn construct_sports_car(&self) -> Car {
        Car::builder()
            .engine(Engine::Gasoline)
            .gps(Gps::Advanced)
            .seats(2)
            .build()
    }

    fn construct_family_car(&self) -> Car {
        Car::builder()
            .engine(Engine::Diesel)
            .gps(Gps::Basic)
            .seats(5)
            .build()
    }
}

fn main() {
    let director = Director;
    let sports_car = director.construct_sports_car();
    let family_car = director.construct_family_car();

    println!("{:#?}", sports_car);
    println!("{:#?}", family_car);
}
