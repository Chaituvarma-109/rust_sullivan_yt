pub mod name_helpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
    
        full_name
    }
}

pub mod database {

}

pub mod private_fns {
    pub fn get_age_plus_five(age: u16) -> u16 {
        age + 5
    }
}
