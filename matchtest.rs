pub fn test_match_string() -> u32 {
    let car_manufacturer: &str = "Hyundai";

    match car_manufacturer {
        "Hyundai" => 30000,
        "Porsche" => 90000,
        _ => 0,
    }
}

pub fn test_match_array() {
    let prices: [u32; 4] = [30000, 50000, 90000, 120000];

    match prices[0..=3] {
        [30000, 50000] => println!("You have some reasonably priced cars"),
        [30000, 50000, ..] => println!("You have variety of cars"),
        _ => println!("You dont have any reasonably priced cars"),
    }
}

pub fn test_match_int() {
    let my_age: u16 = 34;

    let y: u8 = 5;

    match my_age {
        0  => println!("Your are an infant"),
        1..=35 if y == 5 => println!("Your age is upto 35, y is 5"),
        1..=35 if y != 5 => println!("Your age is upto 35, y is not 5"),
        1..=35 => println!("Your age is upto 35"),
        36..=149 => println!("Your age is upto 149"),
        150.. => println!("Your age is above 150"),
    }
}
