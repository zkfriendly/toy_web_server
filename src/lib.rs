use std::thread;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(|| f());
        println!("Executing a function");
    }
}
