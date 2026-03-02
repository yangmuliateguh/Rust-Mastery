use std::thread;

fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_prime_in_range(start: u32, end: u32) -> Vec<u32> {
    (start..end)
        .filter(|&n| is_prime(n))
        .collect()
}

fn main(){
    let ranges = vec![
        (1, 25000),
        (25001, 50000),
        (50001, 75000),
        (75001, 100000)
    ];

    let mut handles = vec![];

    for (start, end) in ranges {
        let handle = thread::spawn(move || {
            find_prime_in_range(start, end)
        });
        handles.push(handle);
    }

    let mut all_primes = vec![];
    for handle in handles {
        let primes = handle.join().unwrap();
        all_primes.extend(primes);
    }

    println!("Total bilangan prima ditemukan: {}", all_primes.len());
    println!("10 bilangan prima pertama: {:?}", &all_primes[..100]);
}