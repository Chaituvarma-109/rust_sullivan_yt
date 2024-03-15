extern crate chrono;
use std::time::{ Duration, Instant};

use chrono::NaiveDate;

pub fn test_stdtime() {
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(14500);
    println!("{}", dur2.as_nanos());

    let dur3 = dur1.checked_sub(dur2);
    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();

    std::thread::sleep(Duration::from_millis(200));

    println!("Elapsed time: {}", now.elapsed().as_micros());
}

pub fn test_chrono() {
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now.format("%Y %b %d %H"));

    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Z %Y %b %d %H"));

    let date1 = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Sun);
    let unwrapped_date = date1.unwrap();
    println!("{}", unwrapped_date.format("Day of Year is: %j"));

    unwrapped_date.iter_days().take(4).for_each(|d| println!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2024, 366);
    println!("{}", date2.unwrap().format("%A %B %d"));

    let birthday = NaiveDate::parse_from_str("2024|||09||15", "%Y|||%m||%d");
    // println!("{:#?}", birthday.err().unwrap());
    println!("{:#?}", birthday.ok().unwrap());

}
