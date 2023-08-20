pub fn factors(n: u64) -> Vec<u64> {
    let mut cur_n = n;
    let mut cur_prime = 2;
    let mut factors = vec![];

    while cur_n != 1 {
        if cur_n % cur_prime == 0 {
            cur_n /= cur_prime;
            factors.push(cur_prime);
            if cur_prime * cur_prime > cur_n {
                break;
            }
        } else {
            cur_prime += 1;
        }
    }
    if cur_n != 1 {
        factors.push(cur_n);
    }
    factors
}
