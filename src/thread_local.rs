#[cfg(test)]
mod tests {
    use std::{cell::RefCell, thread};

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handler = thread::spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = "Budi".to_string();
            });

            NAME.with_borrow(|name| {
                println!("name: {}", name); // this thread will have  Budi as NAME
            });
        });

        let _ = handler.join();

        NAME.with_borrow_mut(|name| {
            println!("name {}", name); // this one stays with default
        });
    }
}
