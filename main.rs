// use crate::closures::test_closures;

pub mod helpers;
pub mod closures;
pub mod matchtest;
pub mod optiontest;
pub mod structs;
pub mod test_traits;
pub mod basic_vectors;
pub mod hash_collections;
pub mod hashset;
pub mod myiters;
pub mod mydatetime;
pub mod mythreads;
pub mod myscopedthreads;
pub mod mymutex;
pub mod mychannels;
pub mod myfiles;
pub mod mycli;

fn main() {
    println!("hello world!!");
    // test_func()
    // let my_result = helpers::name_helpers::get_full_name("chaitu", "rakesh");
    // let new_age = helpers::private_fns::get_age_plus_five(5);
    // println!("Hello from {0}", my_result);
    // println!("New age: {0}", new_age);

    // let value = 5 as f32 / 3 as f32;
    // println!("5 divided by 3 is: {:.2}", value);

    // test_if();
    // test_while();
    // test_loop();
    // test_for();
    // test_closures();
    // matchtest::test_match_array();
    // let result = optiontest::test_option_type();
    // println!("{0}", result.unwrap());

    // let res = optiontest::test_option_string();
    // println!("Name is {0}", res.unwrap());

    // let char_results = optiontest::test_option_chartype();
    // if char_results.is_some() {
    //     println!("User has selected a character type");
    //     println!("Character type selected is: {}", char_results.unwrap().to_string());
    // }
    // else {
    //     println!("Character type is None");
    // }
    // structs::test_create_person();
    // structs::test_create_vehicle();
    // structs::test_create_vehicletuple();

    // test_traits::create_person();

    // basic_vectors::test_vec_int();
    // basic_vectors::test_vec_string();
    // basic_vectors::test_vec_car();

    // hash_collections::test_hashmap_basic();
    // hashset::test_hashset_type();

    // myiters::test_myiters();
    
    // mydatetime::test_stdtime();
    // mydatetime::test_chrono();

    // mythreads::test_threads();
    // mythreads::spawn_thread();

    // myscopedthreads::test_thread_variables();
    // mymutex::test_mutex();

    // mychannels::test_channels();

    // myfiles::test_files();
    // myfiles::test_create_files()
    // myfiles::test_remove_dir();
    // myfiles::test_read_file();

    mycli::test_cli();
    
}

#[allow(dead_code, unused_variables)]
fn test_for() {
    let arr = [1, 2, 3, 5, 6, 7, 8];

    for value in arr {
        println!("{}", value);
    }
}

#[allow(dead_code, unused_variables)]
fn test_loop() {
    let mut x = 1;

    loop {
        if x == 18 {
            println!("Your are eligible for driving license");
            break;
        }

        x += 1;
    }
}

#[allow(dead_code, unused_variables)]
fn test_while() {
    let driving_age = 18u8;

    let mut current_age = 0u8;

    current_age += 1;

    while current_age < driving_age {
        if current_age == driving_age {
            println!("{}", current_age);
            break;
        }
    }
}

#[allow(dead_code, unused_variables)]
fn test_if() {
    let age_to_drive = 18u8;

    println!("Enter persons age: ");
    let my_input = &mut String::from("");
    std::io::stdin().read_line(my_input).unwrap();

    let age: u8 = my_input.replace("\n", "").parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!("Issuing drivers license!");
    } else {
        println!("You are not eligible")
    }

    let driver_license = if age >= 18 { true } else { false };
}

#[allow(dead_code, unused_variables)]
fn test_func(){
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;
    println!("{:?}", y);

    let mut iamold = true;
    iamold = false;
    println!("{:?}", iamold);

    let my_str = 'A';
    let my_str1 = "A";
    println!("{}", my_str);
    println!("{}", my_str1);

    let mut first_name = "Trevor";
    first_name = "Bob";
    println!("{}", first_name);

    let arr = ("Chaitanya", "Varma", 45 as u8);
    println!("{:?}", arr);

    let ages: [u16; 6] = [40, 45, 50, 55, 60, 65];
    println!("{:?}", ages);

    let ages_slice = &ages[1..4];
    println!("{:?}", ages_slice)
}
