pub fn test_myiters() {
    let fruit_names = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];

    let nut_names = vec!["Walnut", "Almonds", "Pecans", "Brazil Nuts"];

    let mut fruit_iter = fruit_names.iter();
    // let nut_iter = nut_names.iter();

    // for fruit in fruit_iter {
    //     println!("{}", fruit);
    // }

    fruit_iter.next();
    fruit_iter.next();
    let item01 = fruit_iter.next();
    println!("First item in iterator is: {}", item01.unwrap());

    let aggregate_foods = fruit_names.iter().chain(&nut_names);

    let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

    for food in aggregate_foods {
        println!("Eating {}", food);
    }

    let fruit_lst_strings = fruit_names.iter().map(|e| String::from(*e));
    let new_fruits = fruit_lst_strings.map(|mut e| { e.push_str(" fruit"); return e;});

    new_fruits.clone().for_each(|e| println!("{}", e));

    println!("last fruit name is: {}", new_fruits.clone().last().unwrap());

    let mut stepby = new_fruits.clone().step_by(2);
    println!("step: {}", stepby.next().unwrap());
    println!("step: {}", stepby.next().unwrap());
    println!("step: {}", stepby.next().unwrap());

    let first_fruit_names = vec!["Trevor", "Shannon", "James", "Tasha"];
    let first_fruit_names_strings = first_fruit_names.iter().map(|e| String::from(*e));

    let last_fruit_names = vec!["Jones", "Sullivan", "Tanner", "Redman"];
    let last_fruit_names_strings = last_fruit_names.iter().map(|e| String::from(*e));

    let full_names = first_fruit_names_strings.zip(last_fruit_names_strings);
    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));

    // for (index, value) in full_names.enumerate() {
    //     println!("index: {0}, value: {1} {2}", index, value.0, value.1);
    // }

    // full_names.skip(1).for_each(|e| println!("Did not skip: {}", e.0));

    // full_names.skip(2).take(1).for_each(|e| println!("take: {}", e.0));

    let foods = vec![("potatoes", 10), ("strawberries", 25), ("burgers", 55)];
    
    let food_quantity = foods.iter().fold(0u32, |a, e| a + e.1);
    println!("Total food quantity is: {}", food_quantity);

    foods.iter().peekable().next();
    let food_iter = foods.iter();
    let mut mypeekable = food_iter.peekable();
    mypeekable.next();
    let food = mypeekable.peek();
    println!("Peeking at {}", food.unwrap().0);


}