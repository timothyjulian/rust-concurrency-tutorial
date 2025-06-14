fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};


    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("{}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application finish");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_handle() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 1..=5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            return counter;
        });

        let result = handle.join();
        match result {
            Ok(counter) => println!("total counter: {}", counter),
            Err(err) => println!("error: {:?}", err),
        }

        println!("application DONE");
    }
}