use std::thread;

fn main() {
    for i in 1..10 {
        let hundle = thread::spawn(move || {
            println!("Hello from thread numbwe {}", i);
        });
        let _ = hundle.join();
    }
}