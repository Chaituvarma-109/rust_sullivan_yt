use std::cell::Cell;

#[derive(Debug)]
enum VehicleColor {
    Silver,
    Blue,
    Red,
    Black,
    White,
    Green,
}

// #[derive(Debug)]
struct VehicleTuple(String, String, u16);

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle{manufacturer: "default".to_string(), model: "default".to_string(), year: 1990, color: VehicleColor::Silver};
        return new_vehicle;
    }
}

struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    meters_walked: u32,
}

impl Person<'static> {
    fn meters_walk(&mut self, meters: u32){
        self.meters_walked += meters;
    }
}

fn new_vehicletuple() -> VehicleTuple {
    return VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2015);
}

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle{
        manufacturer: "Porsche".to_string(),
        model: "911".to_string(),
        year: 1991,
        color: VehicleColor::Green,
    };

    v1.paint(VehicleColor::White);

    v1
}

fn new_person() -> Person<'static> {
    let p1 = Person{
        first_name: Cell::from("Chaitanya"),
        last_name: "Varma".to_string(),
        birth_year: 1986,
        birth_month: 6,
        meters_walked: 0,
    };

    p1.first_name.set("Rakesh");

    p1
}

pub fn test_create_vehicletuple() {
    // println!("Vehicle Tuple: {:?}", new_vehicletuple());
    let my_new_vehicletuple = new_vehicletuple();
    println!("Manufacturer: {0}, model: {1}, year: {2}", my_new_vehicletuple.0, my_new_vehicletuple.1, my_new_vehicletuple.2);
}

pub fn test_create_vehicle() {
    // let my_vehicle = new_vehicle();
    let mut my_vehicle = Vehicle::create_vehicle();
    my_vehicle.paint(VehicleColor::Red);
    println!("{:?}", my_vehicle);
}

pub fn test_create_person() {
    let mut my_person = new_person();
    my_person.meters_walk(8);
    my_person.meters_walk(12);
    println!("Person name is {0} {1} birth year is {2} and month {3}, miles walked {4}", my_person.first_name.get(), my_person.last_name, my_person.birth_year, my_person.birth_month, my_person.meters_walked);
}
