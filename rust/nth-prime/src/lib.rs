pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut cur_nb = 3;

    loop {
        if is_prime(&primes, cur_nb) {
            primes.push(cur_nb);

            if primes.len() > n as usize {
                break;
            }
        }
        cur_nb += 2;
    }
    primes[n as usize]
}

fn is_prime(primes: &[u32], nb: u32) -> bool {
    let cap = nb / 2; // This could be improved

    for &prime in primes {
        if prime > cap {
            break;
        }
        if nb % prime == 0 {
            return false;
        }
    }
    true
}
