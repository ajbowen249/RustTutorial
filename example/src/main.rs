use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let size: usize = 1000000;
    let max_threads = 100;
    let print = true;

    let half_size: usize = size / 2;
    let non_primes = Box::new(vec![false;size]);
    let non_primes_mutex = Arc::new(Mutex::new(non_primes));

    let mut i = 2;

    loop {
        let mut threads = vec![];
        for _ in 0..max_threads {
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
                        }
                        Err(err) => println!("non-primes acccess error {}", err),
                    }
                }
            }));

            i = i + 1;
        }

        for thread in threads {
            let _ = thread.join();
        }

        if i >= half_size {
            break;
        }
    }

    let mut num_primes = 0;

    match non_primes_mutex.lock() {
        Ok(non_primes_value) => {
            for i in 2..(*non_primes_value).len() {
                if !(*non_primes_value)[i] {
                    if print {
                        println!("{}", i);
                    }

                    num_primes = num_primes + 1
                }
            }
        }
        Err(err) => println!("non-primes acccess error {}", err),
    }

    println!("{} primes between 0 and {}.", num_primes, size);
}
