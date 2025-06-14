#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread, time::Duration};

    use tokio::runtime::Runtime;

    // async contract is provided but cannot be executed by default
    // we need Runtime/ Executor to use the async
    // example tokio, async_std, smol
    async fn get_async_data() -> String {
        // async code need to be called by async code
        thread::sleep(Duration::from_secs(2));
        println!("Hello from async");
        return "Hello from async".to_string();
    }

    async fn get_database_data(wait: u64) -> String {
        println!("{:?} : get database data", thread::current().id());
        tokio::time::sleep(Duration::from_secs(wait)).await; // DON'T use thread sleep for tokio Task
        println!("{:?} : hello database data", thread::current().id());
        return "Hello from database".to_string();
    }

    async fn run_concurrent(runtime: Arc<Runtime>) {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = runtime.spawn(get_database_data(i));
            handlers.push(handler);
        }

        for handler in handlers {
            let data = handler.await.unwrap();
            println!("{}", data);
        }
    }

    #[tokio::test]
    async fn test_async() {
        let fut = get_async_data(); // Future is lazy data type
        println!("Finish Call Async");
        let data = fut.await; // to execute Future, use await. Different with join() from thread
        println!("{}", data)
    }

    #[tokio::test]
    async fn test_task() {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = tokio::spawn(get_database_data(i)); // this will spawn Tokio Task
            // Task is a lightweight Thread model (like Goroutines or Java Virtual Thread)
            // This Task can switch Thread execution
            handlers.push(handler);
        }

        for handler in handlers {
            let data = handler.await.unwrap();
            println!("{}", data);
        }
    }

    #[test]
    fn test_tokio_runtime() {
        // tokio runtime must be build inside a non async function
        let runtime = Arc::new(
            // Arc is neede because you are passing the runtime itself into something that executes on the runtime.
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_time()
                .build()
                .unwrap(),
        ); // will create tokio runtime with 10 real thread
        runtime.block_on(run_concurrent(Arc::clone(&runtime)));
    }
}
