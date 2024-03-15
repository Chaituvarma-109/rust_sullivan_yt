use std::fs;

pub fn test_read_file() {
    let file_to_read = "./data/file02.txt";

    let read_result = std::fs::read(file_to_read);

    let convert_bytes_to_string = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        return a;
    };

    if read_result.is_ok() {
        println!("Data read {:?}", read_result.unwrap().iter().fold(String::from(""), convert_bytes_to_string));
    }
}

pub fn test_remove_dir() {
    let path = "./data";
    _ = std::fs::remove_dir_all(path);
}

pub fn test_create_files() {
    let path = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";
    let text = "Trevor Sullivan";
    let text2 = "Nancy Drew";
    let text3 = "Shannon Jones";
    _ = std::fs::write(path, text);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

    // _ = std::fs::remove_file(path2);
}

pub fn test_files() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Directory already exists! Skipping creation ...");
        return;
    }
    let dir_res = fs::create_dir("./data");

    if dir_res.is_ok() {
        println!("Created new data directory.");
    } else {
        println!("Some problem occurred creating data directory. {:?}", dir_res.err());
    }
}
