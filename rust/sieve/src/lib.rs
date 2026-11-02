pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve = vec![true; upper_bound as usize + 1];

    for i in 2..=((upper_bound as f64).sqrt() as usize) {
        if sieve[i] {
            for j in (i * i..=upper_bound as usize).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, &is_prime)| is_prime.then_some(i as u64))
        .collect()
}
