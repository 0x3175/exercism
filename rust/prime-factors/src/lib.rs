pub fn factors(n: u64) -> Vec<u64> {
    let (mut n, mut i) = (n, 2);
    let mut res = Vec::new();

    while n > 1 {
        if n % i == 0 {
            n /= i;
            res.push(i);
        } else {
            i += 1;
        }
    }
    res
}
