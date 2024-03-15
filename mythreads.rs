use std::thread::spawn;

pub fn test_threads() {
    let mut x = 0u128;

    for i in 1..500 {
        x += i;
    }
    println!("Main Thread is finished a little of work ... let's go to handle threads");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;

        for i in 1..5000 {
            x += i;
        }
        println!("{x}");
    };

    println!("Threading Started");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Threading Completed");

    // test_threads();

    loop {
        test_threads();
        
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, let's get out of here");
            break;
        }
    }

    // handle.join();
    // handle2.join();
}
