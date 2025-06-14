#[cfg(test)]
mod tests {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicI32, Ordering},
        },
        thread,
    };

    #[test]
    fn test_race_condition() {
        static mut COUNTER: i32 = 0;

        let mut handlers = vec![];
        for _ in 1..10 {
            let handler = thread::spawn(|| unsafe {
                for _ in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("{}", unsafe { COUNTER });
    }

    #[test]
    fn test_atomic() {
        // need to use static so can be accessed by multiple thread without removing ownership
        static COUNTER: AtomicI32 = AtomicI32::new(0); // can use atomic to ensure atomicity

        let mut handlers = vec![];
        for _ in 1..10 {
            let handler = thread::spawn(|| {
                for _ in 0..1000000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("{}", COUNTER.load(Ordering::Relaxed));
    }

    #[test]
    fn test_atomic_reference() {
        let counter = Arc::new(AtomicI32::new(0)); // Arc is thread-safe Rc which allows multiple shared reference

        let mut handlers = vec![];
        for _ in 1..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("{}", counter.load(Ordering::Relaxed));
    }
}
