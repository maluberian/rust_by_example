use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

const NTHREADS: i32 = 32;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut threads = Vec::new();

    for id in 1..NTHREADS {
        let t_tx = tx.clone();
        let c = thread::spawn(move || {
            t_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
        threads.push(c);
    }

    let mut ids = Vec::new();
    for _ in 1..NTHREADS {
        ids.push(rx.recv());
    }

    for t in threads {
        t.join().unwrap();
    }
    println!("{:?}", ids);
}
