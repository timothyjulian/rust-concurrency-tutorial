#[cfg(test)]
mod tests {
    use std::{sync::Once, thread};

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INT.call_once(|| {
                TOTAL_COUNTER += 1;
            });
            return TOTAL_COUNTER;
        }
    }

    #[test]
    fn test_once() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = thread::spawn(|| {
                let total = get_total();
                println!("total: {}", total);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }
}
