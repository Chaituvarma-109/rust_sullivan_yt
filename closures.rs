struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures() {
    let add = |x: i8, y: i8| {
        println!("x: {} y: {}", x, y);
        x + y
    };
    let result = add(5, 3);

    let print_result = |x: i8| println!("Result: {}", (result + x));
    print_result(93);

    let mut p1 = Person{first_name: "Trevor".to_string(), last_name: "Sullivan".to_string()};
    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("Jones");
    change_name("Chaitanya");
    println!("{}", p1.last_name);
}
