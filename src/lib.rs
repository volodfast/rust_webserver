use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {} got a job; executing.", id);

      job();
    });

    Worker { id, thread }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
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

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      let worker = Worker::new(id, Arc::clone(&receiver));

      workers.push(worker);
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}
