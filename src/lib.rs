use std::thread;

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize) -> Self {
    let thread = thread::spawn(|| {});

    Worker { id, thread }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of workers in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is not greater than 0
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      let worker = Worker::new(id);

      workers.push(worker);
    }

    ThreadPool { workers }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
