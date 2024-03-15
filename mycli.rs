pub fn test_cli() {
    let my_args: Vec<String> = std::env::args().collect();

    println!("{:?}", my_args);

    if my_args.len() != 3 {
        println!("Hey you didn't specify two arguments");
        return ;
    }

    // let name = my_args.by_ref().skip(1).take(1).next().unwrap();
    // let year_born = my_args.by_ref().skip(1).take(1).next().unwrap();

    let name: String = my_args.get(1).unwrap().into();
    let parsed_year = my_args.get(2).unwrap().parse::<i32>();
    if !parsed_year.is_ok() {
        println!("Specified dog year was invalid. Please specify an integer 32 value.");
        return;
    }
    let year_born = parsed_year.ok().unwrap();

    println!("{name} {year_born}");

    let dog01 = new_dog(name, year_born);
    dog01.get_details();
}

fn new_dog(name: String, year_born: i32) -> Dog {
    return Dog { name, year_born };
}

struct Dog {
    name: String,
    year_born: i32,
}

impl Dog {
    fn get_details(&self) {
        println!("Dog name is {} and was born in year {}", self.name, self.year_born);
    }
}
