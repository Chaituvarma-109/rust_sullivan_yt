use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_lst: HashMap<String, f32> = HashMap::new();
    println!("{:?}", stock_lst.len());
    println!("{:?}", stock_lst.is_empty());

    stock_lst.insert("NVDA".to_string(), 478.52);
    stock_lst.insert("AAPL".to_string(), 232.92);
    stock_lst.insert("AMSC".to_string(), 50.78);

    stock_lst.insert("AAPL".to_string(), 233.42);

    stock_lst.entry("META".to_string()).or_insert(346.34);
    stock_lst.entry("META".to_string()).or_insert(358.34);

    println!("{:#?}", stock_lst);

    stock_lst.remove(&("AAPL".to_string()));

    println!("{:#?}", stock_lst);

    for (ticker, curr_val) in stock_lst {
        println!("{} is trading at {}", ticker, curr_val);
    }
}
