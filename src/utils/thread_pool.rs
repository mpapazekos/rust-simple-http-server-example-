use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// ============================================================================

pub struct ThreadPool {

    workers: Vec<ThreadWorker>,
    sender: mpsc::Sender<Message>,
}

struct ThreadWorker {

    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

enum Message {

    BeginMission(Mission),
    AbortMission,
}

// ============================================================================

impl ThreadWorker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {

        let thread = thread::spawn(move || loop {
            
            // The temporary MutexGuard returned from the lock method 
            //  is dropped as soon as the let Mission statement ends. 
            // This ensures that the lock is held during the call to recv, 
            //  but it is released before the call to mission(), 
            //  allowing multiple requests to be serviced concurrently.
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {

                Message::BeginMission(mission) => {
                    
                    println!("Worker {} got a mission; executing.", id);
                    mission();
                }

                Message::AbortMission => {

                    println!("Worker {} was told to abort current mission.", id);
                    break;
                }
            }     
        });

        Self{id, thread: Some(thread)}
    }
}

impl ThreadPool {

    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics!
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        
        let mut workers = Vec::with_capacity(size);  
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {

            // create some workers and store them in a vector
            workers.push(ThreadWorker::new(id, Arc::clone(&receiver)));
        }

        Self {workers, sender}
    }

    /// Send a new mission to workers
    /// 
    /// The func is a closure which contains the mission details
    /// 
    /// # Panics!
    /// 
    /// The 'execute' function will panic if the receiving end
    /// of the channel could not be found
    pub fn execute<F>(&self, func: F) 
    where F: FnOnce() + Send + 'static {

        let mission = Box::new(func);

        self.sender.send(Message::BeginMission(mission)).unwrap();
    }
}

impl Drop for ThreadPool {
    
    fn drop(&mut self) { 
 
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::AbortMission).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {

            println!("Shutting down worker: No. {}", worker.id);
            if let Some(thread) = worker.thread.take() {

                thread.join().unwrap();
            }
        }
    }
}
