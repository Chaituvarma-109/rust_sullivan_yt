pub fn test_vec_int() {
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(45);
    v.push(12);
    v.push(67);
    v.push(100);
    
    println!("{:?}", v);
    println!("{}", v.len());
    println!("{}", v.capacity());
    println!("{}", v[0]);
    println!("{:?}", v.as_slice());
    println!("{:?}", &v.as_slice()[1..=3]);
    println!("{:?}", v.get(8));
    println!("{:?}", v.get(2));

}

pub fn test_vec_string() {
    let str_vec = vec!["chaitanya", "rakesh", "john", "murphy"];

    for name in str_vec.as_slice() {
        println!("{}", name);
    }

    println!("{:?}", str_vec);
}

#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_car() {
    let mut car_list = vec![];
    let mut car_lot2 = vec![];

    for _ in 1..=100u8 {
        car_list.push(
            Car{manufacturer: "Porsche".to_string(), model: "Panamera".to_string()}
        );
    }

    for _ in 1..=100u8 {
        car_lot2.push(
            Car{manufacturer: "Hyundai".to_string(), model: "Sonata".to_string()}
        );
    }

    car_list.append(&mut car_lot2);
    car_list.insert(0, Car { manufacturer: "Tata".to_string(), model: "Nexon".to_string() });
    car_list.remove(0);
    car_list.retain(|e: &Car| {if e.manufacturer == "Porsche" {return true;} else { return false;}} );
    car_list.reserve(5000);

    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_list.capacity());

    println!("{:?}", car_lot2);
    println!("{:?}", car_lot2.len());
    println!("{:?}", car_lot2.capacity());

    println!("{:?}", car_list.get(0).unwrap());

    let mut input = "".to_string();
    std::io::stdin().read_line(&mut input).expect("something bad has happened");
}
