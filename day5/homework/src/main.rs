use std::sync::{Arc, Mutex};
use std::thread;

fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {

    let num_threads = 8;
    let mut_primes = Arc::new(Mutex::new(Vec::new()));

    let threads: Vec<_> = (0..num_threads)
        .map(|_| {
            let primes = Arc::clone(&mut_primes);
            thread::spawn(move || {
                let mut local_primes = Vec::new();
                for n in 2..1000000 {
                    if is_prime(n) {
                        local_primes.push(n);
                    }
                }
                let mut guard = primes.lock().unwrap();
                guard.extend(local_primes);
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    let primes = mut_primes.lock().unwrap();
    println!("{}", primes.len());
}