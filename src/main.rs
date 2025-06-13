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
        thread::sleep(Duration::from_secs(3));
    }
}