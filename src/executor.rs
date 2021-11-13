use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use typemap::{Key, TypeMap};

struct Queue(VecDeque<Box<dyn Fn(&TypeMap) + Send>>);

#[derive(Clone)]
pub struct Executor {
    queue: Arc<Mutex<Queue>>,
}

impl Executor {
    pub fn new()->Self {
        Executor {
            queue: Arc::new(Mutex::new(Queue(VecDeque::new())))
        }
    }
    pub fn execute<F, T: Send + 'static>(&self, mut t: T, map: TypeMap, f: F) -> T
    where
        F: FnOnce(Self, &mut T) + Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let ex = self.clone();
        let thread = std::thread::spawn(move || {
            f(ex, &mut t);
            tx.send(true).unwrap();
            t
        });

        loop {
            if let Ok(_) = rx.recv_timeout(std::time::Duration::from_millis(1)) {
                break;
            }
            let queue = &mut self.queue.lock().unwrap().0;
            for fun in queue.drain(..) {
                fun(&map);
            }
        }
        thread.join().unwrap()
    }

    pub fn run<G, F, H: Send + 'static>(&self, f: F) -> H
    where
        F: Fn(&G) -> H + Send + 'static,
        G: 'static
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let fun = move |map: &TypeMap| {
            let g: &G = map.get::<Mirror<G>>().unwrap();
            let h = f(g);
            if let Err(x) = tx.send(h) {
                println!("{}", x);
            }
        };

        self.queue.lock().unwrap().0.insert(0, Box::new(fun));

        rx.recv().unwrap()
    }
}

struct Mirror<T: 'static> {
    _t: T
}

impl<T> Key for Mirror<T> {
    type Value = T;
}

