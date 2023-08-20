pub fn factors(n: u64) -> Vec<u64> {
    let mut cur_n = n;
    let mut primes = vec![2];
    let mut cur_prime = 2;
    let mut cur_cap = (n as f64).sqrt() as u64;
    let mut factors = vec![];

    while cur_n != 1 {
        if cur_n % cur_prime == 0 {
            cur_n /= cur_prime;
            cur_cap = (cur_n as f64).sqrt() as u64;
            factors.push(cur_prime);
        } else {
            cur_prime = next_prime(&primes);
            primes.push(cur_prime);

            if cur_prime > cur_cap {
                factors.push(cur_n);
                break;
            }
        }
    }
    factors
}

fn next_prime(primes: &[u64]) -> u64 {
    let mut cur_nb: u64 = match primes.last().unwrap() {
        nb if *nb == 2 => return 3,
        nb => *nb + 2,
    };

    while !is_prime(primes, cur_nb) {
        cur_nb += 2;
    }
    cur_nb
}

fn is_prime(primes: &[u64], nb: u64) -> bool {
    let cap = (nb as f64).sqrt() as u64;

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
