use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let size: usize = 4099;
    let half_size: usize = size / 2;
    let non_primes = Box::new(vec![false;size]);

    let mut threads = vec![];
    let non_primes_mutex = Arc::new(Mutex::new(non_primes));

    for i in 2..half_size {
        let non_primes_mutex = non_primes_mutex.clone();
        threads.push(thread::spawn(move || {
            let base = i as usize;
            let mut current = base * 2;
            loop {
                if current >= size {
                    break;
                }

                match non_primes_mutex.lock() {
                    Ok(mut non_primes_value) => {
                        (*non_primes_value)[current] = true;
                        current = current + base;
                    },
                    Err(err) => println!("non-primes acccess error {}", err),
                }
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let mut num_primes = 0;
    
    match non_primes_mutex.lock() {
        Ok(non_primes_value) => {
            for i in 2..(*non_primes_value).len() {
                if !(*non_primes_value)[i] {
                    println!("{}", i);
                    num_primes = num_primes + 1
                }
            }
        },
        Err(err) => println!("non-primes acccess error {}", err),
    }

    println!("{} primes between 0 and {}.", num_primes, size);
}
