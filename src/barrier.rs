#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Barrier},
        thread,
        time::Duration,
    };

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10)); // Barrier will hold all thread execution until desired number of thread
        let mut handlers = vec![];
        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            thread::sleep(Duration::from_secs(1));
            let handler = thread::spawn(move || {
                println!("Join Gamer-{}", i);
                barrier_clone.wait();
                println!("Gamer-{} Start!", i);
            });

            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }
}
