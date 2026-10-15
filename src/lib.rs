use std::{
    println, sync::{Arc, Mutex, mpsc}, thread,
};

pub struct ThreadPool{
    jobs: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker{
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)->Self{
        println!("Start {reciever:?}");
        let thread = thread::spawn(move||{
            loop{
                let job = reciever.lock().unwrap().recv().unwrap();
                println!("Executing Job {id}");
                job();
            };});
        Self{id, thread}
    }
}

impl ThreadPool{
    pub fn new(size:usize)->Self{
        assert!(size>0, "Need at least one thread!");
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..size{
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }
        Self{jobs: workers, sender: tx}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for job in self.jobs.drain(..) {
            println!("Shutting down worker {}", job.id);
            job.thread.join().unwrap();
        }

    }
}