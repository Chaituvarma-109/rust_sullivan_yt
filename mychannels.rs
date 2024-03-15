use std::{sync::mpsc, thread, time::Duration};

pub fn test_channels() {
    let (tx, rx) = mpsc::channel::<u8>();
    // drop(rx);

    rx.recv_timeout(Duration::from_millis(300));

    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("Receive result is: {}", receive_result.is_ok());
    // println!("Receive result is: {}", receive_result.unwrap());

    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("Receive result is: {}", receive_result.is_ok());
    // println!("Receive result is: {}", receive_result.unwrap());

    let processor_code = move || {
        println!("Starting processor thread ...");
        let mut failed_count = 0u8;
        loop {
            println!("Attempting to rx msg from channel ...");
            let rx_res = rx.recv_timeout(Duration::from_millis(200));
            
            if rx_res.is_ok() {
                println!("Received message: {}", rx_res.unwrap());
            }
            else {
                failed_count += 1;
            }

            if failed_count > 5 {
                println!("Aborting processor thread ... no work available");
                break;
            }
        }
    };

    for x in 1..=6 {
        let send_result = tx.send(x);
        println!("Send status: {}", send_result.is_ok());
        thread::sleep(Duration::from_millis(200));
    }

    thread::spawn(processor_code).join();

}
