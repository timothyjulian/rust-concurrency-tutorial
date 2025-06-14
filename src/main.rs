mod channels;

mod atomicity;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 1..=5 {
            println!(
                "[{}-{:?}] counter: {}",
                current.name().unwrap_or_default(),
                current.id(),
                i
            );
            thread::sleep(Duration::from_secs(i));
            counter += 1;
        }

        counter
    }

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

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("total: {}", result1 + result2);
        println!("application finished");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(calculate);
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join().unwrap_or_default();
        let result2 = handle2.join().unwrap_or_default();

        println!("total: {}", result1 + result2);
        println!("application finished");
    }

    #[test]
    fn test_closure() {
        let name = String::from("Timothy");
        let closure = move || {
            //need to use move here to move the value to the closure scope
            thread::sleep(Duration::from_secs(1));
            println!("Hello, {}", name);
        };

        let handler = thread::spawn(closure);
        handler.join().unwrap();

        // println!("{}", name);
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handler = factory
            .spawn(calculate)
            .expect("Failed to create a new thread");

        let total = handler.join().unwrap_or_default();
        println!("{}", total);
    }
}
