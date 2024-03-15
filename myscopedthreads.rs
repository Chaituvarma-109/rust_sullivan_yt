use std::thread;

struct Person {
    first_name: String,
}

pub fn test_thread_variables() {
    let age = 34;
    let p1 = Person{first_name: String::from("Trevor")};

    // let print_age = move || {
    //     println!("Your age is {age}");
    //     println!("Your name is: {}", p1.first_name);
    // };

    let print_age = || {
        println!("This is child clousre");
        println!("Your age is {age}");
        println!("Your name is: {}", &p1.first_name);
    };

    // let _result = thread::spawn(print_age).join();
    thread::scope(|scope| {
        scope.spawn(print_age);
    });

    println!("Your age is {age}");
    println!("Your name is: {}", p1.first_name);

    println!("Finished printing age");
}
