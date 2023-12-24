#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn main() {

    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);
    
    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        }else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }else {
        if roof {
            println!("Build a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        }else {
            println!("Build a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }

    return Car {
        color: String::from(color),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    };
}

fn car_quality(miles: u32) -> (Age, u32) {

    let quality: (Age, u32) = if miles > 0 {(Age::Used, miles)}else {(Age::New, miles)};

    return quality;
}
