use std::thread;
const NTHREADS: usize = 500;

fn main() {
    let mut threads = vec![];

    for i in 0..NTHREADS {
        threads.push(thread::spawn(move || {
            for j in 0..200 {
                println!("thread {} number {}", i, j);
            }
        }))
    }

    for t in threads {
        let _ = t.join();
    }
}