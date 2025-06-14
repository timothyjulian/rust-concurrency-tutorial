#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn test_channel() {
        let (sender, receiver) = mpsc::channel::<String>();

        let handler1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            let test = sender.send("Hello world from handler 1".to_string());
            println!("{:?}", test);
        });

        let handler2 = thread::spawn(move || {
            let message = receiver.recv().unwrap_or_default(); // this will wait for sender
            println!("{}", message)
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    #[test]
    fn test_multiple_data_channel() {
        let (sender, receiver) = mpsc::channel::<String>();

        let handler1 = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender.send(format!("Hello from handler1 {}", i));
            }
            let _ = sender.send("Exit".to_string());
        });

        let handler2 = thread::spawn(move || {
            for value in receiver.iter() {
                // this will wait for sender but only receive 1, therefore need loop, when sender is killed, reciever will also be killed
                println!("{}", value);
            }

            // when reciever is killed, and sender try to send message, will be error
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    #[test]
    fn test_multiple_sender() {
        let (sender, receiver) = mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handler1 = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender.send(format!("Hello from handler1 {}", i));
            }
            let _ = sender.send("Exit".to_string());
        });

        let handler1_add = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender2.send(format!("Hello from handler1_add {}", i));
            }
            let _ = sender2.send("Exit".to_string());
        });

        let handler2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handler1.join();
        let _ = handler1_add.join();
        let _ = handler2.join();
    }
}
